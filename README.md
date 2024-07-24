# rust_orm_gen

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub issues](https://img.shields.io/github/issues/tomblanchard312/rust_orm_gen)](https://github.com/tomblanchard312/rust_orm_gen/issues)
[![GitHub stars](https://img.shields.io/github/stars/tomblanchard312/rust_orm_gen)](https://github.com/tomblanchard312/rust_orm_gen/stargazers)
[![ReadMe](https://img.shields.io/badge/ReadMe-018EF5?logo=readme&logoColor=fff)](https://github.com/tomblanchard312/rust_orm_gen/readme.md)
[![Read the Docs](https://img.shields.io/badge/Read%20the%20Docs-8CA1AF?logo=readthedocs&logoColor=fff)](https://github.com/tomblanchard312/rust_orm_gen/documentation/rust_orm_gen_documentation.pdf)

rust_orm_gen is a Rust library designed to reverse engineer PostgreSQL databases and automatically generate Rust structs and CRUD operations. This tool simplifies the process of interacting with a PostgreSQL database in Rust, ensuring that your code is clean, maintainable, and efficient.

## Features

- Reverse engineer PostgreSQL databases
- Automatically generate Rust structs
- Create CRUD operations for each table
- Easy integration with existing Rust projects

## Installation

Add rust_orm_gen to your `Cargo.toml`:

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

use crate::context::DbContext;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let db_context = DbContext::new(&database_url).await?;
    let output_dir = "db";
    db_context.reverse_engineer(output_dir).await?;
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
use rust_orm_gen::{DbContext, PostgresConnectionManager};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_context = DbContext::new(&database_url).await?;
    db_context.reverse_engineer("db", "Tom Blanchard", "https://github.com/tomblanchard312/rust_orm_gen").await?;

    Ok(())
}
```

## Documentation

 Detailed documentation for rust_orm_gen can be found in the documentation folder at the project root. The main documentation file is named "rust_orm_gen_documentation.pdf".
