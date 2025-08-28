# Development Setup Guide

## 🛠️ Prerequisites

### Required Software
- **Rust**: 1.70+ (latest stable recommended)
- **PostgreSQL**: 9.5+ (for testing and development)
- **Git**: For version control
- **Cargo**: Rust package manager (included with Rust)

### Optional Tools
- **VS Code** with Rust extensions
- **PostgreSQL client** (psql, pgAdmin, etc.)
- **Docker** (for containerized PostgreSQL)

## 🚀 Initial Setup

### 1. Clone the Repository
```bash
git clone https://github.com/tomblanchard312/rust_orm_gen.git
cd rust_orm_gen
```

### 2. Install Rust Dependencies
```bash
cargo build
```

### 3. Set Up Environment
```bash
# Copy environment template
cp .env.example .env  # if .env.example exists

# Or create .env manually
echo "DATABASE_URL=postgres://user:password@localhost/rust_orm_gen_dev" > .env
echo "RUST_LOG=debug" >> .env
```

## 🗄️ Database Setup

### Option 1: Local PostgreSQL Installation

#### Windows
```bash
# Using Chocolatey
choco install postgresql

# Or download from https://www.postgresql.org/download/windows/
```

#### macOS
```bash
# Using Homebrew
brew install postgresql
brew services start postgresql
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

### Option 2: Docker PostgreSQL
```bash
# Create PostgreSQL container
docker run --name rust_orm_gen_db \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=rust_orm_gen_dev \
  -p 5432:5432 \
  -d postgres:15

# Connect to container
docker exec -it rust_orm_gen_db psql -U postgres -d rust_orm_gen_dev
```

### 3. Create Development Database
```sql
-- Connect to PostgreSQL
psql -U postgres

-- Create database and user
CREATE DATABASE rust_orm_gen_dev;
CREATE USER rust_dev WITH PASSWORD 'password';
GRANT ALL PRIVILEGES ON DATABASE rust_orm_gen_dev TO rust_dev;

-- Connect to development database
\c rust_orm_gen_dev

-- Create sample tables for testing
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    title VARCHAR(200) NOT NULL,
    content TEXT,
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert sample data
INSERT INTO users (first_name, last_name, email) VALUES
    ('John', 'Doe', 'john@example.com'),
    ('Jane', 'Smith', 'jane@example.com');

INSERT INTO posts (user_id, title, content, published) VALUES
    (1, 'First Post', 'This is my first post content.', true),
    (2, 'Hello World', 'Hello world post content.', false);
```

## 🔧 Development Workflow

### 1. Code Structure
```
src/
├── lib.rs              # Library entry point
├── main.rs             # CLI application
├── context.rs          # Database context
├── generator.rs         # Code generation
├── crud.rs             # CRUD operations
├── query_builder.rs     # SQL query building
├── metadata.rs          # Database introspection
├── relationships.rs     # Table relationships
├── migrations.rs        # Database migrations
├── validation.rs        # Data validation
├── cache.rs            # Caching system
├── lazy_loading.rs     # Lazy loading
├── transactions.rs      # Transaction management
├── error.rs            # Error handling
├── db.rs               # Database connections
├── cli.rs              # CLI interface
└── models.rs           # Base traits
```

### 2. Running the Project
```bash
# Check compilation
cargo check

# Build the project
cargo build

# Run with default settings
cargo run

# Run with specific command
cargo run generate-schema "postgres://user:pass@localhost/db"

# Run in release mode
cargo run --release
```

### 3. Testing
```bash
# Run all tests
cargo test

# Run specific test module
cargo test --lib relationships

# Run tests with output
cargo test -- --nocapture

# Run tests with logging
RUST_LOG=debug cargo test

# Run tests in release mode
cargo test --release
```

### 4. Code Quality
```bash
# Format code
cargo fmt

# Check code style
cargo clippy

# Run linter with warnings
cargo clippy -- -W clippy::all

# Check for security issues
cargo audit

# Review dependencies
cargo tree
cargo outdated
```

## 📝 Development Guidelines

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and single-purpose
- Use appropriate error types and propagation

### Error Handling
```rust
// Use custom error types
use crate::error::OrmError;

pub async fn process_data() -> Result<Data, OrmError> {
    let connection = get_connection().await?;
    let data = fetch_data(&connection).await?;
    Ok(data)
}

// Proper error conversion
impl From<tokio_postgres::Error> for OrmError {
    fn from(err: tokio_postgres::Error) -> Self {
        OrmError::DatabaseError(err)
    }
}
```

### Testing Patterns
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_functionality() {
        // Setup
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        // Test logic
        let result = test_function(&database_url).await;
        assert!(result.is_ok());
    }
}
```

### Async/Await Usage
```rust
// Use async functions for database operations
pub async fn get_user(id: i32) -> Result<User, OrmError> {
    let client = get_client().await?;
    let row = client.query_one("SELECT * FROM users WHERE id = $1", &[&id]).await?;
    
    Ok(User {
        id: row.get("id"),
        name: row.get("name"),
    })
}

// Proper error handling in async contexts
pub async fn process_users() -> Result<(), OrmError> {
    let users = get_all_users().await?;
    
    for user in users {
        if let Err(e) = process_user(&user).await {
            log::error!("Failed to process user {}: {}", user.id, e);
        }
    }
    
    Ok(())
}
```

## 🧪 Testing Strategy

### Unit Tests
- Test individual functions in isolation
- Mock external dependencies when possible
- Use `#[cfg(test)]` for test-only code
- Test both success and error cases

### Integration Tests
- Test database interactions with real PostgreSQL
- Use test database with sample data
- Clean up test data after tests
- Test complete workflows

### Test Database Setup
```rust
// Test database connection
async fn setup_test_db() -> tokio_postgres::Client {
    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    let (client, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
        .await
        .expect("Failed to connect to test database");
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Test database connection error: {}", e);
        }
    });
    
    client
}

// Test data cleanup
async fn cleanup_test_data(client: &tokio_postgres::Client) {
    client.execute("DELETE FROM posts", &[]).await.ok();
    client.execute("DELETE FROM users", &[]).await.ok();
}
```

## 🔍 Debugging

### Logging
```rust
use log::{info, warn, error, debug};

// Set log level in .env
// RUST_LOG=debug

// Use appropriate log levels
debug!("Processing user: {:?}", user);
info!("User processed successfully: {}", user.id);
warn!("User data incomplete: {}", user.id);
error!("Failed to process user: {}", user.id);
```

### Debug Mode
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests with debug logging
RUST_LOG=debug cargo test

# Build with debug symbols
cargo build --debug
```

### Common Debugging Commands
```bash
# Check database connection
psql -h localhost -U postgres -d rust_orm_gen_dev -c "SELECT 1;"

# View PostgreSQL logs
tail -f /var/log/postgresql/postgresql-*.log

# Check Rust version
rustc --version
cargo --version

# Check dependencies
cargo tree
cargo outdated
```

## 📦 Building and Distribution

### Local Build
```bash
# Development build
cargo build

# Release build
cargo build --release

# Check binary size
ls -lh target/release/rust_orm_gen
```

### Documentation
```bash
# Generate documentation
cargo doc

# Open documentation in browser
cargo doc --open

# Check documentation coverage
cargo doc --document-private-items
```

### Publishing
```bash
# Check package
cargo package

# Publish to crates.io (requires account)
cargo publish

# Create git tag for release
git tag v0.1.4
git push origin v0.1.4
```

## 🚨 Troubleshooting

### Common Issues

#### 1. Database Connection Errors
```bash
# Check PostgreSQL service
sudo systemctl status postgresql

# Check port availability
netstat -an | grep 5432

# Test connection manually
psql -h localhost -U postgres -d rust_orm_gen_dev
```

#### 2. Compilation Errors
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build

# Check for conflicting dependencies
cargo tree -d
```

#### 3. Test Failures
```bash
# Check test database
psql -h localhost -U postgres -d rust_orm_gen_dev -c "\dt"

# Run single test
cargo test test_name -- --nocapture

# Check environment variables
echo $DATABASE_URL
```

#### 4. Performance Issues
```bash
# Profile with cargo
cargo install cargo-flamegraph
cargo flamegraph

# Check memory usage
cargo build --release
valgrind --tool=massif target/release/rust_orm_gen
```

## 🤝 Contributing

### Pull Request Process
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Code Review Checklist
- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code follows project style guidelines
- [ ] Documentation is updated
- [ ] Error handling is appropriate
- [ ] Performance considerations addressed
- [ ] Security implications reviewed
- [ ] Dependencies are up-to-date and secure

### Issue Reporting
- Use GitHub issues for bug reports
- Include reproduction steps
- Provide environment details
- Attach relevant logs and error messages

## 📚 Additional Resources

### Rust Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Database Resources
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [tokio-postgres Documentation](https://docs.rs/tokio-postgres/)

### Development Tools
- [VS Code Rust Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [cargo-edit](https://github.com/killercup/cargo-edit)
- [cargo-watch](https://github.com/passcod/cargo-watch)

---

*Development Setup Guide for rust_orm_gen v0.1.3*
