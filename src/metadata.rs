use tokio_postgres::Client;
use crate::error::OrmError;

pub async fn get_tables(client: &Client) -> Result<Vec<String>, OrmError> {
    let rows = client
        .query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'", &[])
        .await?;
    Ok(rows.iter().map(|row| row.get(0)).collect())
}

pub async fn get_columns(client: &Client, table_name: &str) -> Result<Vec<(String, String)>, OrmError> {
    let query = format!(
        "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = $1"
    );
    let rows = client.query(&query, &[&table_name]).await?;
    Ok(rows.iter().map(|row| (row.get(0), row.get(1))).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use dotenv::dotenv;
    use std::env;
    use crate::db::PostgresConnectionManager;

    #[tokio::test]
    async fn test_get_tables() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = PostgresConnectionManager::new(database_url);
        let client = manager.connect().await.expect("Failed to connect to database");

        let tables = get_tables(&client).await;
        assert!(tables.is_ok(), "Failed to get tables: {:?}", tables.err());
    }

    #[tokio::test]
    async fn test_get_columns() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = PostgresConnectionManager::new(database_url);
        let client = manager.connect().await.expect("Failed to connect to database");

        let columns = get_columns(&client, "your_table_name").await;
        assert!(columns.is_ok(), "Failed to get columns: {:?}", columns.err());
    }
}