use tokio_postgres::{Client, Transaction};
use std::future::Future;
use std::pin::Pin;

pub struct TransactionManager<'a> {
    client: &'a mut Client,
}

impl<'a> TransactionManager<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        TransactionManager { client }
    }

    pub async fn run<F, T, E>(&mut self, f: F) -> Result<T, Box<dyn std::error::Error>>
    where
        F: for<'b> FnOnce(&'b mut Transaction<'b>) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'b>>,
        E: std::error::Error + 'static,
    {
        let mut transaction = self.client.transaction().await?;
        let result = f(&mut transaction).await;

        match result {
            Ok(value) => {
                transaction.commit().await?;
                Ok(value)
            },
            Err(e) => {
                transaction.rollback().await?;
                Err(Box::new(e))
            }
        }
    }
}
