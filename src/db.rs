use tokio_postgres::{Client, NoTls, Error};


pub struct PostgresConnectionManager {
    database_url: String,
}

impl PostgresConnectionManager {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }

    pub async fn connect(&self) -> Result<Client, Error> {
        let (client, connection) = tokio_postgres::connect(&self.database_url, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        Ok(client)
    }

    pub async fn is_valid(&self, client: &Client) -> Result<(), Error> {
        client.simple_query("SELECT 1").await?;
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
    async fn test_connect() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = PostgresConnectionManager::new(database_url);
        let client = manager.connect().await.expect("Failed to connect to database");
        let result = manager.is_valid(&client).await;
        assert!(result.is_ok());
    }
}
