use dotenv::dotenv;
use std::env;
use log::{info, error};
use rust_orm_gen::db::PostgresConnectionManager;
use rust_orm_gen::context::DbContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let (database_url, output_dir) = if args.len() == 3 {
        (args[1].clone(), args[2].clone())
    } else {
        (env::var("DATABASE_URL").expect("DATABASE_URL must be set"), "db".to_string())
    };

    let manager = PostgresConnectionManager::new(database_url.clone());
    
    match manager.connect().await {
        Ok(_conn) => info!("Successfully connected to the database"),
        Err(e) => {
            error!("Failed to connect to the database: {:?}", e);
            return Err(e.into());
        }
    }

    let db_context = match DbContext::new(&database_url).await {
        Ok(context) => context,
        Err(e) => {
            error!("Failed to create DbContext: {:?}", e);
            return Err(e.into());
        }
    };

    match db_context.reverse_engineer(&output_dir, "Tom Blanchard", "https://github.com/tomblanchard312/rust_orm_gen").await {
        Ok(_) => info!("Successfully reverse engineered the database"),
        Err(e) => {
            error!("Failed to reverse engineer the database: {:?}", e);
            return Err(e.into());
        }
    }

    Ok(())
}