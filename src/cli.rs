use serde_json::json;
use std::error::Error;
use thiserror::Error;
use tokio_postgres::NoTls;
use crate::context::DbContext;
use crate::metadata::{get_tables, get_columns};

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Database connection error: {0}")]
    DatabaseConnection(#[from] tokio_postgres::Error),

    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid database URL: {0}")]
    InvalidDatabaseUrl(String),
    
    #[error("ORM generation error: {0}")]
    OrmGeneration(String),
}
pub async fn run_migrations(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Example: Read migration files from a directory and apply them
    for entry in std::fs::read_dir("./migrations")? {
        let path = entry?.path();
        if path.extension().map_or(false, |ext| ext == "sql") {
            let migration_sql = std::fs::read_to_string(path)?;
            client.execute(&migration_sql, &[]).await?;
        }
    }
    Ok(())
}

pub async fn get_schema_json(database_url: &str) -> Result<String, CliError> {
    let db_context = DbContext::new(database_url).await
        .map_err(|e| CliError::DatabaseConnection(e))?;
    
    let conn = db_context.pool.get().await
        .map_err(|e| CliError::DatabaseConnection(e.into()))?;

    let tables = get_tables(&conn).await?;

    let mut schema = Vec::new();

    for table in tables {
        let columns = get_columns(&conn, &table).await?;
        let table_info = json!({
            "name": table,
            "columns": columns
        });
        schema.push(table_info);
    }

    serde_json::to_string_pretty(&schema)
        .map_err(CliError::JsonSerialization)
}

pub async fn run_cli() -> Result<(), CliError> {
    let database_url = std::env::args().nth(1)
        .ok_or_else(|| CliError::InvalidDatabaseUrl("No database URL provided".to_string()))?;

    let schema_only = std::env::args().any(|arg| arg == "--schema-only");

    if schema_only {
        let schema_json = get_schema_json(&database_url).await?;
        println!("{}", schema_json);
    } else {
        let db_context = DbContext::new(&database_url).await?;
        let conn = db_context.pool.get().await?;
        generate_orm_files(&conn, "output", "Tom Blanchard", "https://github.com/tomblanchard312/rust_orm_gen")
            .await
            .map_err(|e| CliError::OrmGeneration(e.to_string()))?;
        println!("ORM files generated successfully.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_postgres::Config;
    use std::str::FromStr;

    // Helper function to create a test database
    async fn setup_test_db() -> String {
        let config = Config::from_str("host=localhost user=postgres password=password dbname=postgres")
            .expect("Failed to parse config");
        let (client, connection) = config.connect(NoTls).await.expect("Failed to connect");
        tokio::spawn(connection);

        let db_name = format!("test_db_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        client.execute(&format!("CREATE DATABASE {}", db_name), &[]).await.expect("Failed to create test database");

        format!("postgres://postgres:password@localhost/{}", db_name)
    }

    // Helper function to tear down the test database
    async fn teardown_test_db(db_url: &str) {
        let config = Config::from_str("host=localhost user=postgres password=password dbname=postgres")
            .expect("Failed to parse config");
        let (client, connection) = config.connect(NoTls).await.expect("Failed to connect");
        tokio::spawn(connection);

        let db_name = db_url.split('/').last().unwrap();
        client.execute(&format!("DROP DATABASE {}", db_name), &[]).await.expect("Failed to drop test database");
    }

    #[tokio::test]
    async fn test_get_schema_json() {
        let db_url = setup_test_db().await;

        // Create a test table
        let config = Config::from_str(&db_url).expect("Failed to parse config");
        let (client, connection) = config.connect(NoTls).await.expect("Failed to connect");
        tokio::spawn(connection);

        client.execute("CREATE TABLE test_table (id SERIAL PRIMARY KEY, name TEXT NOT NULL)", &[])
            .await
            .expect("Failed to create test table");

        // Test get_schema_json
        let schema_json = get_schema_json(&db_url).await.expect("Failed to get schema JSON");
        let schema: serde_json::Value = serde_json::from_str(&schema_json).expect("Failed to parse schema JSON");

        assert!(schema.is_array());
        assert_eq!(schema.as_array().unwrap().len(), 1);
        assert_eq!(schema[0]["name"], "test_table");
        assert!(schema[0]["columns"].is_array());
        assert_eq!(schema[0]["columns"].as_array().unwrap().len(), 2);

        teardown_test_db(&db_url).await;
    }

    #[tokio::test]
    async fn test_invalid_db_url() {
        let result = get_schema_json("invalid_url").await;
        assert!(result.is_err());
        match result {
            Err(CliError::DatabaseConnection(_)) => (),
            _ => panic!("Expected DatabaseConnection error"),
        }
    }
}