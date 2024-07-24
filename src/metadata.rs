use tokio_postgres::Client;

pub async fn get_tables(client: &Client) -> Result<Vec<String>, tokio_postgres::Error> {
    let rows = client
        .query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'", &[])
        .await?;
    Ok(rows.iter().map(|row| row.get(0)).collect())
}

pub async fn get_columns(client: &Client, table_name: &str) -> Result<Vec<(String, String)>, tokio_postgres::Error> {
    let query = format!(
        "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '{}'",
        table_name
    );
    let rows = client.query(&query, &[]).await?;
    Ok(rows.iter().map(|row| (row.get(0), row.get(1))).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_get_tables() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let (client, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
            .await
            .expect("Failed to connect to database");
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        let tables = get_tables(&client).await;
        assert!(tables.is_ok());
    }

    #[tokio::test]
    async fn test_get_columns() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let (client, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
            .await
            .expect("Failed to connect to database");
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        let columns = get_columns(&client, "your_table_name").await;
        assert!(columns.is_ok());
    }
}
