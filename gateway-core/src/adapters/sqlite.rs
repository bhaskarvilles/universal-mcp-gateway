use async_trait::async_trait;
use serde_json::{Map, Number, Value};
use sqlx::{Column, Row, SqlitePool, TypeInfo, ValueRef};
use std::collections::{HashMap, HashSet};

use super::{Adapter, AdapterError, ExecutionContext, ParameterProperty, Tool, ToolParameters};
use crate::config::SourceConfig;

const DEFAULT_LIMIT: u64 = 100;
const MAX_LIMIT: u64 = 1_000;

/// Exposes tables in a SQLite database as bounded, read-only MCP tools.
#[allow(dead_code)]
pub struct SQLiteAdapter {
    name: String,
    connection_string: String,
    configured_tables: Vec<String>,
    pool: Option<SqlitePool>,
    tables: HashSet<String>,
    tools: Vec<Tool>,
}

impl SQLiteAdapter {
    pub fn new(config: SourceConfig) -> Result<Self, AdapterError> {
        let connection_string = config
            .config
            .get("connection")
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InitializationError(
                    "Missing 'connection' in SQLite adapter config".to_string(),
                )
            })?
            .to_string();
        if !connection_string.starts_with("sqlite:") {
            return Err(AdapterError::InitializationError(
                "SQLite connection must use a sqlite: URL".to_string(),
            ));
        }
        let configured_tables = config
            .config
            .get("tables")
            .and_then(|value| value.as_array())
            .map(|tables| {
                tables
                    .iter()
                    .filter_map(|value| value.as_str().map(ToOwned::to_owned))
                    .collect()
            })
            .unwrap_or_else(|| vec!["*".to_string()]);
        Ok(Self {
            name: config.name,
            connection_string,
            configured_tables,
            pool: None,
            tables: HashSet::new(),
            tools: Vec::new(),
        })
    }

    async fn discover_tables(pool: &SqlitePool) -> Result<Vec<String>, AdapterError> {
        let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name")
            .fetch_all(pool).await
            .map_err(|error| AdapterError::InitializationError(format!("Failed to inspect SQLite schema: {error}")))?;
        rows.into_iter()
            .map(|row| {
                row.try_get::<String, _>("name").map_err(|error| {
                    AdapterError::InitializationError(format!(
                        "Failed to read SQLite table name: {error}"
                    ))
                })
            })
            .collect()
    }

    fn generate_tools(tables: &[String]) -> Vec<Tool> {
        tables
            .iter()
            .map(|table| Tool {
                name: format!("query_{table}"),
                description: format!("Read rows from the SQLite table '{table}'"),
                parameters: ToolParameters {
                    param_type: "object".to_string(),
                    properties: HashMap::from([(
                        "limit".to_string(),
                        ParameterProperty {
                            prop_type: "integer".to_string(),
                            description: Some(format!("Maximum rows to return (1-{MAX_LIMIT})")),
                            default: Some(Value::Number(DEFAULT_LIMIT.into())),
                        },
                    )]),
                    required: Vec::new(),
                },
                returns: Some("array".to_string()),
            })
            .collect()
    }

    fn quoted_identifier(identifier: &str) -> String {
        format!("\"{}\"", identifier.replace('"', "\"\""))
    }

    fn limit(params: &Value) -> Result<u64, AdapterError> {
        let limit = params
            .get("limit")
            .and_then(Value::as_u64)
            .unwrap_or(DEFAULT_LIMIT);
        if !(1..=MAX_LIMIT).contains(&limit) {
            return Err(AdapterError::InvalidParameters(format!(
                "'limit' must be between 1 and {MAX_LIMIT}"
            )));
        }
        Ok(limit)
    }

    fn json_value(row: &sqlx::sqlite::SqliteRow, index: usize) -> Result<Value, AdapterError> {
        let raw = row.try_get_raw(index).map_err(|error| {
            AdapterError::ExecutionError(format!("Failed to read SQLite value: {error}"))
        })?;
        if raw.is_null() {
            return Ok(Value::Null);
        }
        match raw.type_info().name() {
            "INTEGER" => row
                .try_get::<i64, _>(index)
                .map(|value| Value::Number(value.into()))
                .map_err(|error| {
                    AdapterError::ExecutionError(format!(
                        "Failed to convert SQLite integer: {error}"
                    ))
                }),
            "REAL" => row
                .try_get::<f64, _>(index)
                .ok()
                .and_then(Number::from_f64)
                .map(Value::Number)
                .ok_or_else(|| {
                    AdapterError::ExecutionError(
                        "SQLite returned a non-finite floating-point value".to_string(),
                    )
                }),
            "BLOB" => row
                .try_get::<Vec<u8>, _>(index)
                .map(|bytes| {
                    Value::Array(
                        bytes
                            .into_iter()
                            .map(|byte| Value::Number(byte.into()))
                            .collect(),
                    )
                })
                .map_err(|error| {
                    AdapterError::ExecutionError(format!("Failed to convert SQLite blob: {error}"))
                }),
            _ => row
                .try_get::<String, _>(index)
                .map(Value::String)
                .map_err(|error| {
                    AdapterError::ExecutionError(format!("Failed to convert SQLite value: {error}"))
                }),
        }
    }
}

#[async_trait]
impl Adapter for SQLiteAdapter {
    fn name(&self) -> &str {
        &self.name
    }

    async fn initialize(&mut self) -> Result<(), AdapterError> {
        let pool = SqlitePool::connect(&self.connection_string)
            .await
            .map_err(|error| {
                AdapterError::InitializationError(format!("Failed to connect to SQLite: {error}"))
            })?;
        let available_tables = Self::discover_tables(&pool).await?;
        let tables = if self.configured_tables.iter().any(|table| table == "*") {
            available_tables
        } else {
            let available: HashSet<_> = available_tables.iter().cloned().collect();
            for table in &self.configured_tables {
                if !available.contains(table) {
                    return Err(AdapterError::InitializationError(format!(
                        "Configured SQLite table '{table}' does not exist"
                    )));
                }
            }
            self.configured_tables.clone()
        };
        self.tables = tables.iter().cloned().collect();
        self.tools = Self::generate_tools(&tables);
        self.pool = Some(pool);
        Ok(())
    }

    async fn discover_tools(&self) -> Result<Vec<Tool>, AdapterError> {
        Ok(self.tools.clone())
    }

    async fn execute(
        &self,
        tool: &str,
        params: Value,
        _ctx: ExecutionContext,
    ) -> Result<Value, AdapterError> {
        let table = tool
            .strip_prefix("query_")
            .ok_or_else(|| AdapterError::ToolNotFound(tool.to_string()))?;
        if !self.tables.contains(table) {
            return Err(AdapterError::ToolNotFound(tool.to_string()));
        }
        let pool = self.pool.as_ref().ok_or_else(|| {
            AdapterError::ExecutionError("SQLite adapter has not been initialized".to_string())
        })?;
        let query = format!("SELECT * FROM {} LIMIT ?", Self::quoted_identifier(table));
        let rows = sqlx::query(&query)
            .bind(Self::limit(&params)? as i64)
            .fetch_all(pool)
            .await
            .map_err(|error| {
                AdapterError::ExecutionError(format!("SQLite query failed: {error}"))
            })?;
        rows.into_iter()
            .map(|row| {
                let mut record = Map::new();
                for (index, column) in row.columns().iter().enumerate() {
                    record.insert(column.name().to_string(), Self::json_value(&row, index)?);
                }
                Ok(Value::Object(record))
            })
            .collect::<Result<Vec<_>, AdapterError>>()
            .map(Value::Array)
    }

    async fn health_check(&self) -> Result<(), AdapterError> {
        let pool = self.pool.as_ref().ok_or_else(|| {
            AdapterError::ExecutionError("SQLite adapter has not been initialized".to_string())
        })?;
        sqlx::query("SELECT 1")
            .execute(pool)
            .await
            .map_err(|error| {
                AdapterError::ExecutionError(format!("SQLite health check failed: {error}"))
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::sqlite::SqlitePoolOptions;

    fn config(connection: &str) -> SourceConfig {
        SourceConfig {
            name: "inventory".to_string(),
            source_type: "sqlite".to_string(),
            config: HashMap::from([(
                "connection".to_string(),
                Value::String(connection.to_string()),
            )]),
        }
    }

    #[tokio::test]
    async fn discovers_and_queries_tables() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let connection = format!("sqlite://{}", database.path().display());
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&connection)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE products (id INTEGER, name TEXT, price REAL)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO products VALUES (1, 'keyboard', 49.99), (2, 'mouse', 25.0)")
            .execute(&pool)
            .await
            .unwrap();
        let mut adapter = SQLiteAdapter::new(config(&connection)).unwrap();
        adapter.initialize().await.unwrap();
        assert_eq!(
            adapter.discover_tools().await.unwrap()[0].name,
            "query_products"
        );
        let results = adapter
            .execute("query_products", json!({ "limit": 1 }), test_context())
            .await
            .unwrap();
        assert_eq!(
            results,
            json!([{ "id": 1, "name": "keyboard", "price": 49.99 }])
        );
    }

    #[tokio::test]
    async fn rejects_invalid_limits_and_unknown_tools() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let connection = format!("sqlite://{}", database.path().display());
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&connection)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE products (id INTEGER)")
            .execute(&pool)
            .await
            .unwrap();
        let mut adapter = SQLiteAdapter::new(config(&connection)).unwrap();
        adapter.initialize().await.unwrap();
        assert!(matches!(
            adapter
                .execute("query_products", json!({ "limit": 0 }), test_context())
                .await,
            Err(AdapterError::InvalidParameters(_))
        ));
        assert!(matches!(
            adapter
                .execute("query_unknown", json!({}), test_context())
                .await,
            Err(AdapterError::ToolNotFound(_))
        ));
    }

    fn test_context() -> ExecutionContext {
        ExecutionContext {
            user_id: None,
            session_id: "test".to_string(),
            timeout: 30,
            metadata: HashMap::new(),
        }
    }
}
