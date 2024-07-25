use crate::error::OrmError;
use crate::metadata::{get_tables, get_columns};
use crate::generator::generate_struct;
use crate::crud::generate_crud_operations;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use log::{info, error};
use crate::db::PostgresConnectionManager;
use chrono::Utc;

pub struct DbContext {
    pub manager: PostgresConnectionManager,
}

impl DbContext {
    pub async fn new(database_url: &str) -> Result<Self, OrmError> {
        let manager = PostgresConnectionManager::new(database_url.to_string());
        Ok(Self { manager })
    }

    pub async fn reverse_engineer(&self, output_dir: &str, author: &str, github_link: &str) -> Result<(), OrmError> {
        info!("Reverse engineering the database schema");
        let conn = self.manager.connect().await?;
        let tables = get_tables(&conn).await?;
        let date = Utc::now().date_naive();
        for table in tables {
            info!("Processing table: {}", table);
            match get_columns(&conn, &table).await {
                Ok(columns) => {
                    let columns_map: HashMap<String, String> = columns.into_iter().collect();
                    let struct_def = generate_struct(&table, columns_map.clone(), author, github_link, date);
                    let crud_ops = generate_crud_operations(&table, columns_map, author, github_link, date);

                    // Ensure output directory exists
                    fs::create_dir_all(output_dir)?;

                    // Write struct definition to file
                    let struct_file_path = Path::new(output_dir).join(format!("{}.rs", table));
                    fs::write(&struct_file_path, struct_def)
                        .map_err(|e| OrmError::IoError(e))?;

                    // Write CRUD operations to file
                    let crud_file_path = Path::new(output_dir).join(format!("{}_crud.rs", table));
                    fs::write(&crud_file_path, crud_ops)
                        .map_err(|e| OrmError::IoError(e))?;

                    info!("Completed processing table: {}", table);
                }
                Err(e) => error!("Failed to get columns for table {}: {}", table, e),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_reverse_engineer() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_context = DbContext::new(&database_url).await.unwrap();
        let result = db_context.reverse_engineer("db", "Tom Blanchard", "https://github.com/tomblanchard312/rust_orm_gen").await;
        if let Err(e) = &result {
            eprintln!("Reverse engineering failed: {:?}", e);
        }
        assert!(result.is_ok(), "Reverse engineering should succeed");
    }
}