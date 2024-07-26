use dotenv::dotenv;
use std::env;
use log::error;
use rust_orm_gen::migrations::run_migrations;
use rust_orm_gen::generator::generate_structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or_default();

    match command {
        "migrate" => {
            let db_url = args.get(2).expect("Database URL required for migration");
            let client = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?.0;
            let migrations = vec![]; // You need to define your migrations here
            run_migrations(&client, &migrations).await?;
        },
        "generate-schema" => {
            let db_url = args.get(2).expect("Database URL required for schema generation");
            generate_structs(db_url).await?;
        },        
        _ => {
            error!("Unknown command or insufficient arguments");
            std::process::exit(1);
        }
    }

    Ok(())
}
