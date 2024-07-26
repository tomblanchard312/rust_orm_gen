# rust_orm_gen

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub issues](https://img.shields.io/github/issues/tomblanchard312/rust_orm_gen)](https://github.com/tomblanchard312/rust_orm_gen/issues)
[![GitHub stars](https://img.shields.io/github/stars/tomblanchard312/rust_orm_gen)](https://github.com/tomblanchard312/rust_orm_gen/stargazers)
[![ReadMe](https://img.shields.io/badge/ReadMe-018EF5?logo=readme&logoColor=fff)](https://github.com/tomblanchard312/rust_orm_gen/readme.md)
[![Read the Docs](https://img.shields.io/badge/Read%20the%20Docs-8CA1AF?logo=readthedocs&logoColor=fff)](https://github.com/tomblanchard312/rust_orm_gen/documentation/rust_orm_gen_documentation.pdf)

**rust_orm_gen** is a Rust library designed to reverse engineer PostgreSQL databases and automatically generate Rust structs, CRUD operations, and manage database migrations. This tool simplifies the process of interacting with a PostgreSQL database in Rust, ensuring that your code is clean, maintainable, and efficient.

## Features

- Reverse engineer PostgreSQL databases.
- Automatically generate Rust structs.
- Create CRUD operations for each table.
- Handle database migrations and schema changes.
- Define and validate relationships between tables.
- Build and execute complex SQL queries programmatically.
- Validate data models against database constraints.

## Installation

To install rust_orm_gen, add it to your Rust project by modifying your `Cargo.toml` file:

```toml
[dependencies]
rust_orm_gen = { git = "https://github.com/tomblanchard312/rust_orm_gen.git" }
tokio = { version = "1", features = ["full"] }
dotenv = "0.15.0"

## Configuration
 
 Ensure your .env file is correctly configured with the database URL:
 
 ```text
 DATABASE_URL=postgres://user:password@localhost/mydb
 ```

 Replace user, password, and mydb with your actual PostgreSQL credentials and database name.

## Usage

1. Initialize the Database Context:

```rust
mod context;
mod metadata;
mod generator;
mod crud;
mod db;
mod migrations;
mod relationships;
mod query_builder;
mod validation;

use crate::context::DbContext;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let db_context = DbContext::new(&database_url).await?;
    db_context.reverse_engineer("db").await?;
    Ok(())
}

```

2. Run the program:

```sh
cargo run
```

# Include in your code as a crate

In your Cargo.toml file add the following:

```text
[dependencies]
rust_orm_gen = "0.1.0"
```

Add the connection to the Postgresql database in your .env file:

```text
DATABASE_URL=postgres://postgres:yourpassword@localhost/yourdatabase
```

To use the library in your code, import the library in your main.rs file:

```rust
mod db {
    pub mod users;
    pub mod users_crud;
}

use db::users::Users;
use db::users_crud::{create_users, get_users, update_users, delete_users};
use tokio_postgres::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(database_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Example usage of generated ORM code
    let new_user = Users {
        id: 1,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    let created_user = create_users(&client, &new_user).await?;
    println!("Created user: {:?}", created_user);

    let fetched_user = get_users(&client, 1).await?;
    println!("Fetched user: {:?}", fetched_user);

    Ok(())
}
```

## Documentation

 Detailed documentation for rust_orm_gen can be found in the documentation folder at the project root. The main documentation file is named "rust_orm_gen_documentation.pdf".

Changes
version 0.1.2: Added better error handling and the ability to call from the command-line for integration with VSCode extension.
version 0.1.2: Introduced features for handling database migrations, relationships, and complex query building.

Example code:

```sh
 rust_orm_gen "postgres://postgres:password@localhost:5432/yourdb" "C:\GitProjects\rust\rustsample\db"
```

This example will output all of the database structs and methods to the db folder in your project.
