use dotenv::dotenv;
use std::env;
use log::info;
use rust_orm_gen::db::PostgresConnectionManager;
use rust_orm_gen::context::DbContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new(database_url.clone());
    
    let _conn = manager.connect().await?;
    info!("Successfully connected to the database");

    let db_context = DbContext::new(&database_url).await?;
    db_context.reverse_engineer("db", "Tom Blanchard", "https://github.com/tomblanchard312/rust_orm_gen").await?;

    Ok(())
}
