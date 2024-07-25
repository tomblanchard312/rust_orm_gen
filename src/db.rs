use crate::error::OrmError;
use tokio_postgres::{Client, NoTls};
use std::future::Future;

pub trait ConnectionManager {
    fn connect(&self) -> impl Future<Output = Result<Client, OrmError>> + Send;
    fn is_valid<'a>(&'a self, client: &'a Client) -> impl Future<Output = Result<(), OrmError>> + Send + 'a;
}

pub struct PostgresConnectionManager {
    database_url: String,
}

impl PostgresConnectionManager {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }

    pub async fn connect(&self) -> Result<Client, OrmError> {
        let (client, connection) = tokio_postgres::connect(&self.database_url, NoTls)
            .await
            .map_err(|e| OrmError::ConnectionError(e.to_string()))?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(client)
    }

    pub async fn is_valid(&self, client: &Client) -> Result<(), OrmError> {
        client.simple_query("SELECT 1").await
            .map_err(|e| OrmError::QueryError(e.to_string()))?;
        Ok(())
    }
}

impl ConnectionManager for PostgresConnectionManager {
    fn connect(&self) -> impl Future<Output = Result<Client, OrmError>> + Send {
        self.connect()
    }

    fn is_valid<'a>(&'a self, client: &'a Client) -> impl Future<Output = Result<(), OrmError>> + Send + 'a {
        self.is_valid(client)
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
        let result = ConnectionManager::connect(&manager).await;
        
        match result {
            Ok(client) => {
                let valid = ConnectionManager::is_valid(&manager, &client).await;
                assert!(valid.is_ok(), "Connection should be valid");
            },
            Err(e) => {
                panic!("Failed to connect to database: {:?}", e);
            }
        }
    }
}