use tokio_postgres::Client;
use std::error::Error;

pub struct Migration {
    pub version: i32,
    pub up: String,
    pub down: String,
}

pub async fn run_migrations(client: &Client, migrations: &[Migration]) -> Result<(), Box<dyn Error>> {
    // Create migrations table if it doesn't exist
    client.execute("CREATE TABLE IF NOT EXISTS migrations (version INTEGER PRIMARY KEY)", &[]).await?;

    for migration in migrations {
        let version: i32 = client.query_one("SELECT version FROM migrations WHERE version = $1", &[&migration.version]).await?.get(0);
        if version == 0 {
            client.execute(&migration.up, &[]).await?;
            client.execute("INSERT INTO migrations (version) VALUES ($1)", &[&migration.version]).await?;
        }
    }

    Ok(())
}