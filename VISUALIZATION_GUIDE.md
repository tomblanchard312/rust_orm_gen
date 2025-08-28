# 🎨 Enhanced Database Schema Visualization Guide

This guide covers the comprehensive database schema visualization capabilities of `rust_orm_gen`, including interactive features, custom themes, real-time monitoring, and multiple export formats.

## 🚀 Quick Start

### CLI Usage

```bash
# Generate a specific format
cargo run -- visualize postgres://user:pass@localhost/db html output/

# Generate all formats
cargo run -- visualize postgres://user:pass@localhost/db all output/

# Generate specific export formats
cargo run -- visualize postgres://user:pass@localhost/db svg output/
cargo run -- visualize postgres://user:pass@localhost/db pdf output/
cargo run -- visualize postgres://user:pass@localhost/db visio output/
```

### Programmatic Usage

```rust
use rust_orm_gen::{
    SchemaVisualizer, 
    SchemaMonitor,
    visualization::{Table, Column, Relationship, Theme, MonitoringConfig}
};

// Create sample schema
let tables = vec![/* your tables */];
let relationships = vec![/* your relationships */];

// Create visualizer with custom theme
let custom_theme = Theme {
    name: "custom_dark".to_string(),
    primary_color: "#FF6B6B".to_string(),
    secondary_color: "#4ECDC4".to_string(),
    background_color: "#2C3E50".to_string(),
    text_color: "#ECF0F1".to_string(),
    border_color: "#34495E".to_string(),
    font_family: "Segoe UI, Tahoma, Geneva, Verdana, sans-serif".to_string(),
    font_size: 16,
};

let visualizer = SchemaVisualizer::new(tables, relationships)
    .with_theme(custom_theme);

// Generate visualizations
let dot_content = visualizer.generate_dot()?;
let html_content = visualizer.generate_html()?;
let svg_content = visualizer.export_svg()?;
let pdf_content = visualizer.export_pdf()?;
let visio_content = visualizer.export_visio()?;
```

## 📊 Supported Formats

### 1. **Graphviz DOT** (`.dot`)
- **Description**: Text-based graph description language
- **Features**: Hierarchical layout, relationship visualization, custom styling
- **Use Case**: Professional diagrams, publication, automated processing
- **Rendering**: Use Graphviz tools: `dot -Tpng schema.dot -o schema.png`

### 2. **Mermaid** (`.mmd`)
- **Description**: Markdown-inspired diagram syntax
- **Features**: ER diagrams, relationship mapping, GitHub/GitLab integration
- **Use Case**: Documentation, version control, web-based viewing
- **Rendering**: Paste into [Mermaid Live Editor](https://mermaid.live/)

### 3. **PlantUML** (`.puml`)
- **Description**: UML diagram generation from text
- **Features**: Standard UML notation, multiple diagram types
- **Use Case**: Software architecture, system design, documentation
- **Rendering**: Use PlantUML tools or online services

### 4. **Interactive HTML** (`.html`)
- **Description**: Web-based interactive visualization
- **Features**: 
  - 🔍 **Search functionality** - Find tables quickly
  - 🎯 **Zoom controls** - Zoom in/out and reset
  - 🎨 **Theme toggle** - Switch between light/dark themes
  - 📱 **Responsive design** - Works on all devices
  - 🖱️ **Pan functionality** - Middle-click and drag to pan
  - 📊 **Real-time updates** - Timestamp updates every 30 seconds
- **Use Case**: Presentations, team collaboration, interactive exploration

### 5. **JSON Schema** (`.json`)
- **Description**: Structured data representation
- **Features**: Complete schema metadata, configuration settings, theme info
- **Use Case**: API integration, data analysis, tool integration

## 📤 Export Formats

### 6. **SVG Export** (`.svg`)
- **Description**: Scalable vector graphics
- **Features**: High-quality, scalable, web-compatible
- **Use Case**: Web applications, print media, vector graphics

### 7. **PDF Export** (`.pdf`)
- **Description**: Portable document format
- **Features**: Print-ready, cross-platform, professional
- **Use Case**: Reports, documentation, printing

### 8. **Visio Export** (`.vsdx`)
- **Description**: Microsoft Visio format
- **Features**: Professional diagramming, enterprise tools
- **Use Case**: Business documentation, enterprise workflows

## 🎨 Customization & Themes

### Theme Configuration

```rust
let theme = Theme {
    name: "corporate".to_string(),
    primary_color: "#2E86AB".to_string(),      // Main brand color
    secondary_color: "#A23B72".to_string(),    // Accent color
    background_color: "#F8F9FA".to_string(),   // Background
    text_color: "#212529".to_string(),         // Text color
    border_color: "#DEE2E6".to_string(),       // Border color
    font_family: "Arial, sans-serif".to_string(),
    font_size: 14,
};

let visualizer = SchemaVisualizer::new(tables, relationships)
    .with_theme(theme);
```

### Visualization Configuration

```rust
use rust_orm_gen::visualization::VisualizationConfig;

let config = VisualizationConfig {
    theme: Theme::default(),
    show_relationships: true,      // Display foreign key relationships
    show_data_types: true,         // Show column data types
    show_constraints: true,        // Show PK/FK/NOT NULL constraints
    layout_engine: "dot".to_string(), // Layout algorithm
};

let visualizer = SchemaVisualizer::new(tables, relationships)
    .with_config(config);
```

## 🔍 Real-Time Schema Monitoring

### Monitoring Configuration

```rust
let monitoring_config = MonitoringConfig {
    check_interval_seconds: 30,           // Check every 30 seconds
    enable_notifications: true,           // Enable change notifications
    track_schema_changes: true,           // Track overall schema changes
    track_table_changes: true,            // Track table additions/removals
    track_column_changes: true,           // Track column modifications
    track_relationship_changes: true,     // Track relationship changes
};

let monitor = SchemaMonitor::new(monitoring_config);
```

### Start Monitoring

```rust
// Start monitoring with callback
monitor.start_monitoring("postgres://localhost/db", |event| {
    println!("Schema change: {} - {}", 
        event.event_type, event.change_description);
}).await?;

// Monitor is now running in background
// Check status
if monitor.is_monitoring() {
    println!("Monitoring is active");
}

// Get change history
let history = monitor.get_change_history();
for event in history {
    println!("{}: {} ({})", 
        event.timestamp.format("%H:%M:%S"),
        event.change_description,
        event.severity);
}

// Stop monitoring
monitor.stop_monitoring();
```

### Schema Change Events

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaChangeEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: String,           // "schema_check", "table_added", etc.
    pub table_name: Option<String>,   // Affected table (if applicable)
    pub column_name: Option<String>,  // Affected column (if applicable)
    pub change_description: String,   // Human-readable description
    pub severity: String,             // "info", "warning", "error"
}
```

## 🔧 Advanced Features

### Interactive HTML Features

The generated HTML includes several interactive capabilities:

- **Search**: Type in the search box to filter tables
- **Zoom**: Use + and - buttons or mouse wheel to zoom
- **Pan**: Middle-click and drag to pan around the visualization
- **Theme Toggle**: Switch between light and dark themes
- **Export**: Download as SVG or print as PDF
- **Responsive**: Automatically adapts to different screen sizes

### Relationship Auto-Detection

The system automatically detects:
- **Primary Keys**: Marked with (PK) in visualizations
- **Foreign Keys**: Marked with (FK) in visualizations
- **Constraints**: NOT NULL, unique constraints
- **Relationships**: Foreign key relationships between tables

### Performance Optimization

- **Lazy Loading**: Visualizations generated on-demand
- **Caching**: Schema metadata cached for repeated access
- **Background Processing**: Monitoring runs in background threads
- **Memory Management**: Automatic cleanup of old monitoring data

## 📁 Output Structure

```
output/
├── schema.dot          # Graphviz DOT format
├── schema.mmd          # Mermaid format
├── schema.html         # Interactive HTML
├── schema.puml         # PlantUML format
├── schema.json         # JSON schema
├── schema.svg          # SVG export
├── schema.pdf          # PDF export
└── schema.vsdx         # Visio export
```

## 🚀 Integration Examples

### Web Application Integration

```rust
// Generate HTML for web display
let html_content = visualizer.generate_html()?;

// Serve via web framework
app.route("/schema", get(|| async { 
    html_content.clone() 
}));
```

### CI/CD Pipeline Integration

```rust
// Generate diagrams for documentation
let dot_content = visualizer.generate_dot()?;
std::fs::write("docs/schema.dot", dot_content)?;

// Generate PDF for release notes
let pdf_content = visualizer.export_pdf()?;
std::fs::write("release/schema.pdf", pdf_content)?;
```

### Database Migration Monitoring

```rust
// Monitor schema during migrations
let monitor = SchemaMonitor::new(MonitoringConfig::default());

monitor.start_monitoring(database_url, |event| {
    if event.severity == "error" {
        // Alert team of schema issues
        send_alert(&event);
    }
    
    // Log all changes
    log_schema_change(&event);
}).await?;
```

## 🔧 Troubleshooting

### Common Issues

1. **PDF Generation Fails**
   - Ensure `printpdf` dependency is available
   - Check file permissions for output directory

2. **SVG Export Issues**
   - Verify `svg` crate is properly linked
   - Check for invalid characters in table/column names

3. **Monitoring Not Working**
   - Ensure database connection is stable
   - Check monitoring configuration settings
   - Verify callback function is properly defined

### Performance Tips

- Use appropriate check intervals for monitoring (30+ seconds for production)
- Limit change history size for long-running monitoring
- Consider using themes with web-safe fonts for better compatibility
- Generate only needed formats to reduce processing time

## 🔮 Future Enhancements

Planned features include:
- **Live Database Connection**: Real-time schema monitoring with live database
- **Advanced Layouts**: Force-directed, hierarchical, and circular layouts
- **Custom Styling**: CSS customization and advanced theming
- **Collaboration**: Multi-user editing and commenting
- **Version Control**: Schema change history and rollback
- **API Integration**: REST API for programmatic access
- **Plugin System**: Extensible visualization formats

## 📚 Additional Resources

- [Graphviz Documentation](https://graphviz.org/documentation/)
- [Mermaid Syntax](https://mermaid-js.github.io/mermaid/#/syntax/entityRelationshipDiagram)
- [PlantUML Reference](https://plantuml.com/guide)
- [SVG Specification](https://www.w3.org/TR/SVG/)
- [PDF Reference](https://www.adobe.com/content/dam/acom/en/devnet/pdf/pdfs/PDF32000_2008.pdf)

---

For more information, examples, and updates, visit the project repository and documentation.
