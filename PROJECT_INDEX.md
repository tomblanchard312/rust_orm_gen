# Rust ORM Generator - Project Index

## Project Overview
**rust_orm_gen** is a Rust library designed to reverse engineer PostgreSQL databases and automatically generate Rust structs, CRUD operations, and manage database migrations. This tool simplifies the process of interacting with a PostgreSQL database in Rust, ensuring that your code is clean, maintainable, and efficient.

**Version:** 0.1.3  
**License:** MIT  
**Author:** Tom Blanchard (tomblanchard312@gmail.com)  
**Repository:** https://github.com/tomblanchard312/rust_orm_gen

## Project Structure

### Core Files
- **`Cargo.toml`** - Project configuration and dependencies
- **`Readme.md`** - Main project documentation
- **`schema.json`** - Database schema configuration (currently empty)
- **`.gitignore`** - Git ignore patterns (target/, .env)

### Source Code (`src/`)

#### Main Entry Points
- **`main.rs`** - CLI application entry point with commands:
  - `migrate` - Run database migrations
  - `generate-schema` - Generate Rust structs from database schema
- **`lib.rs`** - Library entry point, exports all public modules

#### Core Components

##### Database Management
- **`db.rs`** - Database connection management
  - `PostgresConnectionManager` - Manages PostgreSQL connections
  - `ConnectionManager` trait - Abstract connection interface
  - Connection validation and error handling

##### Context & Orchestration
- **`context.rs`** - Main database context and reverse engineering
  - `DbContext` - Orchestrates the reverse engineering process
  - `reverse_engineer()` - Main function to generate ORM code
  - Handles file generation for structs and CRUD operations

##### Code Generation
- **`generator.rs`** - Rust struct generation from database schema
  - `generate_struct()` - Creates Rust structs from table definitions
  - `generate_structs()` - Batch generation for all tables
  - PostgreSQL to Rust type mapping
  - Header generation with metadata

- **`crud.rs`** - CRUD operation generation
  - `generate_crud_operations()` - Creates CRUD functions for tables
  - Generates: create, read, update, delete, list operations
  - Uses QueryBuilder for SQL generation
  - Consistent code formatting and structure

##### Query Building
- **`query_builder.rs`** - SQL query construction
  - `QueryBuilder` - Main query building interface
  - `Select<T>` - SELECT query builder with joins, conditions, ordering
  - Support for: JOINs, WHERE clauses, ORDER BY, GROUP BY, LIMIT/OFFSET
  - Type-safe parameter binding
  - Aggregate functions (COUNT, SUM, AVG, MIN, MAX)

##### Database Metadata
- **`metadata.rs`** - Database schema introspection
  - `get_tables()` - Retrieves all table names from database
  - `get_columns()` - Gets column information for specific tables
  - Uses PostgreSQL information_schema

##### Relationships
- **`relationships.rs`** - Table relationship management
  - `Relationship` - Defines table relationships
  - `RelationType` - OneToOne, OneToMany, ManyToMany
  - `HasRelationships` trait - Interface for relationship definitions
  - Example implementation for User entity

##### Migrations
- **`migrations.rs`** - Database migration system
  - `Migration` - Migration structure with version, up/down SQL
  - `run_migrations()` - Applies migrations to database
  - Automatic migration tracking table creation

##### Validation
- **`validation.rs`** - Data validation framework
  - `Validate` trait - Async validation interface
  - `ValidateSchema` trait - Schema validation
  - `ValidationResult` - Validation error collection and reporting

##### Caching & Performance
- **`cache.rs`** - Generic caching system
  - `Cache<K, V>` - Thread-safe HashMap-based cache
  - Async read/write operations with RwLock
  - Generic key-value storage

- **`lazy_loading.rs`** - Lazy loading implementation
  - `LazyLoaded<T>` - Lazy initialization of values
  - Thread-safe with Arc<Mutex<Option<T>>>
  - Customizable loader functions

##### Transactions
- **`transactions.rs`** - Database transaction management
  - `TransactionManager` - Wraps PostgreSQL transactions
  - Automatic commit/rollback handling
  - Generic transaction execution with error handling

##### Error Handling
- **`error.rs`** - Custom error types
  - `OrmError` enum - Comprehensive error categorization
  - Database, connection, query, parse, I/O, and environment errors
  - Error conversion implementations

##### CLI Interface
- **`cli.rs`** - Command-line interface utilities
  - `run_cli()` - Main CLI execution
  - `get_schema_json()` - Schema export to JSON
  - `run_migrations()` - Migration execution
  - `generate_schema_visualizations()` - Generate schema visualizations
  - Support for multiple output formats (dot, mermaid, html, plantuml, json, all)
  - Error handling with custom `CliError` types

##### Models
- **`models.rs`** - Base model trait
  - `Model` trait - Defines table name and columns
  - Foundation for generated model structs

##### Schema Visualization
- **`visualization.rs`** - Database schema visualization
  - `SchemaVisualizer` - Generates multiple visualization formats
  - `generate_dot()` - Graphviz DOT format for diagrams
  - `generate_mermaid()` - Mermaid ER diagrams
  - `generate_html()` - Interactive web-based viewer
  - `generate_plantuml()` - PlantUML UML diagrams
  - `generate_json_schema()` - JSON schema export
  - Support for custom styling and themes

## Dependencies

### Core Dependencies
- **tokio** (1.x) - Async runtime with full features
- **tokio-postgres** (0.7) - PostgreSQL client
- **serde** (1.0) - Serialization/deserialization
- **chrono** (0.4) - Date and time handling
- **uuid** (1.0) - UUID generation and handling
- **bigdecimal** (0.2) - Decimal number support

### Development Dependencies
- **mockall** (0.11.3) - Mocking framework for testing
- **env_logger** (0.9) - Environment-based logging
- **log** (0.4) - Logging facade

## Usage Examples

### Basic Usage
```rust
use rust_orm_gen::DbContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_context = DbContext::new("postgres://user:pass@localhost/db").await?;
    db_context.reverse_engineer("db", "Author", "https://github.com/user/repo").await?;
    Ok(())
}
```

### CLI Usage
```bash
# Generate schema
cargo run generate-schema "postgres://user:pass@localhost/db"

# Run migrations
cargo run migrate "postgres://user:pass@localhost/db"
```

### Generated Code Usage
```rust
use db::users::Users;
use db::users_crud::{create_users, get_users, update_users, delete_users};

let new_user = Users {
    id: 1,
    first_name: "John".to_string(),
    last_name: "Doe".to_string(),
};

let created_user = create_users(&client, &new_user).await?;
```

## Features

### Core Features
- ✅ PostgreSQL database reverse engineering
- ✅ Automatic Rust struct generation
- ✅ CRUD operation generation
- ✅ Database migration management
- ✅ Relationship definition and management
- ✅ Type-safe query building
- ✅ Data validation framework
- ✅ Caching and lazy loading
- ✅ Transaction management
- ✅ CLI interface

### Advanced Features
- ✅ Custom type mapping (PostgreSQL → Rust)
- ✅ Join support in queries
- ✅ Aggregate function support
- ✅ Pagination (LIMIT/OFFSET)
- ✅ Error handling and logging
- ✅ Async/await support
- ✅ Thread-safe operations

### 🎨 **Database Schema Visualization** ✨
- ✅ **Multiple Output Formats**: DOT, Mermaid, HTML, PlantUML, JSON
- ✅ **Export Formats**: SVG, PDF, Visio (.vsdx) export capabilities
- ✅ **Interactive HTML**: Web-based schema viewer with search, zoom, pan, and theme toggle
- ✅ **Custom Themes**: Configurable color schemes and styling
- ✅ **Real-time Monitoring**: Schema change detection and event tracking
- ✅ **Relationship Auto-detection**: Automatic foreign key and constraint detection
- ✅ **Graphviz Integration**: Generate vector graphics and diagrams
- ✅ **Mermaid Support**: Documentation-ready ER diagrams
- ✅ **PlantUML Export**: UML diagram generation
- ✅ **JSON Schema**: Data exchange and API documentation
- ✅ **CLI Integration**: Command-line visualization generation
- ✅ **Programmatic API**: Use visualization in your own code

## Architecture Patterns

### Design Principles
1. **Separation of Concerns** - Each module has a specific responsibility
2. **Trait-based Abstractions** - Interfaces for extensibility
3. **Async-first Design** - Built on tokio runtime
4. **Error Propagation** - Comprehensive error handling
5. **Type Safety** - Generic implementations with compile-time checks

### Module Dependencies
```
lib.rs
├── context.rs (orchestrates)
├── generator.rs (code generation)
├── crud.rs (CRUD operations)
├── query_builder.rs (SQL building)
├── metadata.rs (schema introspection)
├── relationships.rs (table relationships)
├── migrations.rs (database migrations)
├── validation.rs (data validation)
├── cache.rs (caching)
├── lazy_loading.rs (lazy initialization)
├── transactions.rs (transaction management)
├── error.rs (error handling)
├── db.rs (connection management)
├── cli.rs (command line interface)
├── models.rs (base traits)
└── visualization.rs (enhanced schema visualization with themes, monitoring, and export formats)
```

## Testing

### Test Coverage
- **Unit Tests**: 9 tests covering core functionality
- **Integration Tests**: Database connection tests (require PostgreSQL)
- **Mock Testing**: Uses mockall for isolated testing

### Test Categories
- ✅ Relationship creation and management
- ✅ Query builder functionality
- ✅ Struct generation
- ✅ CRUD operation generation
- ❌ Database connection tests (require live DB)
- ❌ Metadata retrieval tests (require live DB)

## 🎨 Visualization Usage

### Command Line Visualization

```bash
# Generate Mermaid diagram
cargo run visualize postgres://user:pass@localhost/mydb mermaid

# Generate HTML visualization
cargo run visualize postgres://user:pass@localhost/mydb html

# Generate all formats
cargo run visualize postgres://user:pass@localhost/mydb all

# Generate export formats
cargo run visualize postgres://user:pass@localhost/mydb svg ./diagrams
cargo run visualize postgres://user:pass@localhost/mydb pdf ./diagrams
cargo run visualize postgres://user:pass@localhost/mydb visio ./diagrams

# Specify output directory
cargo run visualize postgres://user:pass@localhost/mydb dot ./diagrams
```

### Programmatic Usage

```rust
use rust_orm_gen::{DbContext, SchemaVisualizer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_context = DbContext::new("postgres://user:pass@localhost/mydb").await?;
    
    // Generate all visualization formats
    let formats = vec!["dot", "mermaid", "html", "plantuml", "json"];
    let files = db_context.generate_visualizations("output", &formats).await?;
    
    for (format, path) in files {
        println!("Generated {}: {}", format, path);
    }
    
    Ok(())
}
```

### Output Formats

- **DOT (.dot)**: Graphviz format for vector graphics
- **Mermaid (.mmd)**: Documentation and web diagrams
- **HTML (.html)**: Interactive web-based viewer with search, zoom, pan, and themes
- **PlantUML (.puml)**: UML diagram generation
- **JSON (.json)**: Schema data exchange
- **SVG (.svg)**: Scalable vector graphics export
- **PDF (.pdf)**: Portable document format export
- **Visio (.vsdx)**: Microsoft Visio format export

### Examples

Check the `examples/` directory for:
- `visualization_example.rs` - Complete working example with themes and monitoring
- Sample database schema with users, posts, and comments
- All visualization formats demonstrated including export formats
- Real-time schema monitoring demonstration
- Custom theme configuration examples

## Configuration

### Environment Variables
- **DATABASE_URL** - PostgreSQL connection string
- **RUST_LOG** - Logging level configuration

### Database Requirements
- PostgreSQL 9.5+ (for modern features)
- Public schema access
- Information schema read permissions

## Development Status

### Current Version: 0.1.3
- **Stability**: Beta/Development
- **Completeness**: Core functionality implemented
- **Testing**: Basic unit tests passing
- **Documentation**: Good coverage with examples

### Known Limitations
- Requires live PostgreSQL database for full testing
- Limited to public schema tables
- Basic relationship support
- No custom type mapping configuration

## Future Enhancements

### Planned Features
- Configuration file support
- Custom type mapping configuration
- More database backends (MySQL, SQLite)
- Advanced relationship handling
- Migration rollback support
- Performance optimization
- Extended CLI options

### Contributing
- MIT licensed
- GitHub repository available
- Issue tracking and pull requests welcome

## Performance Characteristics

### Memory Usage
- Efficient string handling with String types
- Minimal memory overhead for generated code
- Thread-safe caching with Arc<RwLock>

### Async Performance
- Built on tokio runtime for high concurrency
- Non-blocking database operations
- Efficient connection pooling

## Security Considerations

### Database Security
- Parameterized queries prevent SQL injection
- Connection string validation
- Environment variable configuration

### Code Generation
- No arbitrary code execution
- Safe string interpolation
- Validation of generated code structure

### Dependency Security
- ✅ **RESOLVED**: Fixed `atty` crate vulnerability (unaligned pointer dereference on Windows)
- Uses `std::io::IsTerminal` instead of unmaintained `atty` dependency
- Regular security audits recommended via `cargo audit`
- Minimal feature flags to reduce attack surface

## Troubleshooting

### Common Issues
1. **Database Connection Errors**: Check DATABASE_URL and PostgreSQL service
2. **Permission Errors**: Ensure database user has schema access
3. **Type Mapping Issues**: Verify PostgreSQL data types
4. **Compilation Errors**: Check Rust version compatibility
5. **Security Vulnerabilities**: Run `cargo audit` to check for known vulnerabilities

### Debug Mode
```bash
RUST_LOG=debug cargo run
```

## License and Attribution

- **License**: MIT
- **Author**: Tom Blanchard
- **Contributors**: Open source contributors
- **Dependencies**: See Cargo.lock for full dependency tree

---

*This index was generated on 2025-08-27 and covers rust_orm_gen version 0.1.3*
