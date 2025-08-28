# Rust ORM Generator Examples

This directory contains examples demonstrating how to use the `rust_orm_gen` library, particularly its new visualization capabilities.

## 🎨 Visualization Example

### Running the Example

```bash
# Run the visualization example
cargo run --example visualization_example

# Or build and run the binary directly
cargo build --example visualization_example
./target/debug/examples/visualization_example
```

### What It Does

The `visualization_example.rs` demonstrates:

1. **Creating a sample database schema** with users, posts, and comments tables
2. **Defining relationships** between tables (foreign keys)
3. **Generating multiple visualization formats**:
   - **DOT** (Graphviz) - For vector graphics and diagrams
   - **Mermaid** - For documentation and web-based diagrams
   - **PlantUML** - For UML diagrams and technical documentation
   - **JSON** - For data exchange and API documentation
   - **HTML** - For interactive web-based visualization

### Output Files

After running the example, check the `examples/output/` directory for:

- `schema.dot` - Graphviz DOT format
- `schema.mmd` - Mermaid format
- `schema.puml` - PlantUML format
- `schema.json` - JSON schema export
- `schema.html` - Interactive HTML visualization

### Sample Schema

The example creates a blog database with:

- **users** table (id, username, email, created_at)
- **posts** table (id, title, content, user_id, published, created_at)
- **comments** table (id, content, post_id, user_id, created_at)

With relationships:
- posts.user_id → users.id (MANY_TO_ONE)
- comments.post_id → posts.id (MANY_TO_ONE)
- comments.user_id → users.id (MANY_TO_ONE)

## 🚀 Using the Visualizations

### 1. HTML Visualization
```bash
# Open in your default browser
start examples/output/schema.html  # Windows
open examples/output/schema.html   # macOS
xdg-open examples/output/schema.html  # Linux
```

### 2. Graphviz DOT
```bash
# Install Graphviz first
# Windows: winget install graphviz
# macOS: brew install graphviz
# Ubuntu: sudo apt-get install graphviz

# Generate PNG
dot -Tpng examples/output/schema.dot -o examples/output/schema.png

# Generate SVG
dot -Tsvg examples/output/schema.dot -o examples/output/schema.svg
```

### 3. Mermaid
- Visit [Mermaid Live Editor](https://mermaid.live/)
- Paste the contents of `schema.mmd`
- See the diagram rendered in real-time

### 4. PlantUML
- Install VS Code extension "PlantUML"
- Open `schema.puml` in VS Code
- Use Alt+Shift+D to preview

## 🔧 Customizing the Example

### Modify the Schema

Edit the `tables` and `relationships` vectors in `visualization_example.rs`:

```rust
let tables = vec![
    Table {
        name: "your_table".to_string(),
        columns: vec![
            Column {
                name: "id".to_string(),
                data_type: "INTEGER".to_string(),
                is_nullable: false,
                is_primary_key: true,
                is_foreign_key: false,
            },
            // Add more columns...
        ],
    },
    // Add more tables...
];
```

### Add New Relationships

```rust
let relationships = vec![
    Relationship {
        from_table: "table1".to_string(),
        from_column: "foreign_key".to_string(),
        to_table: "table2".to_string(),
        to_column: "primary_key".to_string(),
        relationship_type: "MANY_TO_ONE".to_string(),
    },
    // Add more relationships...
];
```

## 📚 Integration with Real Databases

The visualization functionality can also be used with real PostgreSQL databases:

```bash
# Generate visualizations from a real database
cargo run visualize postgres://user:pass@localhost/mydb mermaid
cargo run visualize postgres://user:pass@localhost/mydb all ./output
```

## 🎯 Use Cases

### Documentation
- Generate ER diagrams for technical documentation
- Create visual schemas for README files
- Document database changes over time

### Presentations
- Export high-quality diagrams for slides
- Create interactive HTML visualizations for demos
- Generate vector graphics for publications

### Development
- Understand complex database schemas
- Plan database migrations
- Communicate schema changes to team members

### Integration
- Use JSON schema for API documentation
- Import into other diagramming tools
- Automate documentation generation

## 🛠️ Troubleshooting

### Common Issues

1. **Graphviz not found**: Install Graphviz using your package manager
2. **Output directory permissions**: Ensure write access to the examples/output directory
3. **Unicode characters**: The visualizations support emojis and special characters

### Getting Help

- Check the main [VISUALIZATION_GUIDE.md](../VISUALIZATION_GUIDE.md)
- Review the [PROJECT_INDEX.md](../PROJECT_INDEX.md)
- Open an issue on GitHub for bugs or feature requests

---

**Happy Visualizing! 🎨✨**
