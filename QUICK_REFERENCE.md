# Rust ORM Generator - Quick Reference

## 🚀 Quick Start

### 1. Add to Cargo.toml
```toml
[dependencies]
rust_orm_gen = { git = "https://github.com/tomblanchard312/rust_orm_gen.git" }
tokio = { version = "1", features = ["full"] }
dotenv = "0.15"
```

### 2. Set Environment Variable
```bash
# .env file
DATABASE_URL=postgres://user:password@localhost/mydb
```

### 3. Basic Usage
```rust
use rust_orm_gen::DbContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::ok();
    let db_context = DbContext::new(&env::var("DATABASE_URL")?).await?;
    db_context.reverse_engineer("db", "Your Name", "https://github.com/user/repo").await?;
    Ok(())
}
```

## 📋 CLI Commands

```bash
# Generate Rust structs from database
cargo run generate-schema "postgres://user:pass@localhost/db"

# Run migrations
cargo run migrate "postgres://user:pass@localhost/db"
```

## 🔧 Core Components

### Database Context
```rust
use rust_orm_gen::DbContext;

let db_context = DbContext::new(database_url).await?;
db_context.reverse_engineer(output_dir, author, github_link).await?;
```

### Query Builder
```rust
use rust_orm_gen::QueryBuilder;

let (query, params) = QueryBuilder::select::<User>()
    .where_clause("age > $1")
    .order_by("name", true)
    .limit(10)
    .build();
```

### Relationships
```rust
use rust_orm_gen::HasRelationships;

impl HasRelationships for User {
    fn relationships() -> Vec<Box<dyn Any>> {
        vec![
            Relationship::new(RelationType::OneToMany, "user_id", "posts"),
        ]
    }
}
```

### Validation
```rust
use rust_orm_gen::Validate;

impl Validate for User {
    async fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        if self.name.is_empty() {
            result.add_error("name", "Name cannot be empty");
        }
        result
    }
}
```

## 📊 Generated Code Structure

### Generated Files
```
db/
├── users.rs          # User struct definition
├── users_crud.rs     # CRUD operations for User
├── posts.rs          # Post struct definition
└── posts_crud.rs     # CRUD operations for Post
```

### Generated Struct Example
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    #[serde(rename = "id")] pub id: i32,
    #[serde(rename = "first_name")] pub first_name: String,
    #[serde(rename = "last_name")] pub last_name: String,
}
```

### Generated CRUD Example
```rust
pub async fn create_users(client: &Client, entity: &Users) -> Result<Users, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::insert::<Users>()
        .values(&[&entity.id, &entity.first_name, &entity.last_name])
        .returning(&["id", "first_name", "last_name"])
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Users {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
    })
}
```

## 🗄️ PostgreSQL Type Mapping

| PostgreSQL | Rust |
|------------|------|
| `integer`, `serial` | `i32` |
| `bigint`, `bigserial` | `i64` |
| `text`, `varchar` | `String` |
| `boolean` | `bool` |
| `date` | `chrono::NaiveDate` |
| `timestamp` | `chrono::NaiveDateTime` |
| `uuid` | `uuid::Uuid` |
| `json`, `jsonb` | `serde_json::Value` |
| `numeric` | `bigdecimal::BigDecimal` |

## 🔍 Database Introspection

### Get Tables
```rust
use rust_orm_gen::metadata::get_tables;

let tables = get_tables(&client).await?;
```

### Get Columns
```rust
use rust_orm_gen::metadata::get_columns;

let columns = get_columns(&client, "users").await?;
```

## 🚦 Error Handling

### Custom Error Types
```rust
use rust_orm_gen::error::OrmError;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(OrmError::DatabaseError(e)) => eprintln!("Database error: {}", e),
    Err(OrmError::ConnectionError(e)) => eprintln!("Connection error: {}", e),
    Err(e) => eprintln!("Other error: {:?}", e),
}
```

## 🧪 Testing

### Run Tests
```bash
# All tests
cargo test

# Specific module
cargo test --lib relationships

# With logging
RUST_LOG=debug cargo test
```

### Test Database Setup
```rust
#[tokio::test]
async fn test_database_operations() {
    dotenv::ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Your test code here
}
```

## 📝 Configuration

### Environment Variables
```bash
DATABASE_URL=postgres://user:pass@localhost/db
RUST_LOG=info
```

### Logging Levels
```bash
RUST_LOG=error    # Only errors
RUST_LOG=warn     # Warnings and errors
RUST_LOG=info     # Info, warnings, and errors
RUST_LOG=debug    # All log levels
RUST_LOG=trace    # Maximum verbosity
```

## 🔄 Migrations

### Migration Structure
```rust
use rust_orm_gen::migrations::Migration;

let migrations = vec![
    Migration {
        version: 1,
        up: "CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT)".to_string(),
        down: "DROP TABLE users".to_string(),
    }
];

run_migrations(&client, &migrations).await?;
```

## 💾 Caching

### Generic Cache Usage
```rust
use rust_orm_gen::cache::Cache;

let cache: Cache<String, User> = Cache::new();
cache.set("user:1".to_string(), user.clone()).await;
let cached_user = cache.get(&"user:1".to_string()).await;
```

### Lazy Loading
```rust
use rust_orm_gen::lazy_loading::LazyLoaded;

let lazy_user = LazyLoaded::new(|| User::default());
let user = lazy_user.get().await;
```

## 🔐 Transactions

### Transaction Management
```rust
use rust_orm_gen::transactions::TransactionManager;

let mut tx_manager = TransactionManager::new(&mut client);
let result = tx_manager.run(|tx| {
    Box::pin(async move {
        // Your transaction logic here
        tx.execute("INSERT INTO users (name) VALUES ($1)", &[&"John"]).await?;
        Ok(())
    })
}).await?;
```

## 📚 Common Patterns

### Batch Operations
```rust
// Generate all tables
let tables = get_tables(&client).await?;
for table in tables {
    let columns = get_columns(&client, &table).await?;
    let struct_def = generate_struct(&table, columns, author, github_link, date);
    // Save to file...
}
```

### Error Propagation
```rust
pub async fn process_user(user: &User) -> Result<(), OrmError> {
    user.validate().await?;
    
    let client = get_client().await?;
    create_user(&client, user).await?;
    
    Ok(())
}
```

## 🔒 Security

### Security Best Practices
```bash
# Check for known vulnerabilities
cargo audit

# Update dependencies
cargo update

# Review dependency tree
cargo tree
```

### Recent Security Fixes
- ✅ **RESOLVED**: `atty` crate vulnerability (Windows unaligned pointer dereference)
- ✅ **RESOLVED**: Replaced with `std::io::IsTerminal` (Rust 1.70+)
- ✅ **RESOLVED**: Disabled unnecessary features in `env_logger`

## 🚨 Troubleshooting

### Common Issues

1. **Connection Refused**
   - Check PostgreSQL service is running
   - Verify DATABASE_URL format
   - Check firewall settings

2. **Permission Denied**
   - Ensure user has schema access
   - Check database permissions
   - Verify connection string

3. **Type Mapping Errors**
   - Check PostgreSQL data types
   - Verify custom type handling
   - Review generated code

### Debug Commands
```bash
# Verbose compilation
cargo build -vv

# Run with debug logging
RUST_LOG=debug cargo run

# Check dependencies
cargo tree
```

## 📖 Further Reading

- [Full Documentation](documentation/rust_orm_gen_documentation.pdf)
- [Project Index](PROJECT_INDEX.md)
- [GitHub Repository](https://github.com/tomblanchard312/rust_orm_gen)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)

---

*Quick Reference for rust_orm_gen v0.1.3*
