use rust_orm_gen::{
    SchemaVisualizer, 
    SchemaMonitor,
    visualization::{Table, Column, Relationship, Theme, MonitoringConfig}
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🎨 Enhanced Database Schema Visualization Example");
    println!("===============================================");

    // Create sample database schema
    let tables = create_sample_schema();
    let relationships = create_sample_relationships();

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

    let visualizer = SchemaVisualizer::new(tables.clone(), relationships.clone())
        .with_theme(custom_theme);

    // Generate all visualization formats
    println!("\n📊 Generating visualizations...");
    
    let output_dir = "examples/output";
    std::fs::create_dir_all(output_dir)?;

    // Generate basic formats
    let dot_content = visualizer.generate_dot()?;
    std::fs::write(format!("{}/schema.dot", output_dir), dot_content)?;
    println!("✅ Generated DOT format");

    let mermaid_content = visualizer.generate_mermaid()?;
    std::fs::write(format!("{}/schema.mmd", output_dir), mermaid_content)?;
    println!("✅ Generated Mermaid format");

    let html_content = visualizer.generate_html()?;
    std::fs::write(format!("{}/schema.html", output_dir), html_content)?;
    println!("✅ Generated HTML format");

    let plantuml_content = visualizer.generate_plantuml()?;
    std::fs::write(format!("{}/schema.puml", output_dir), plantuml_content)?;
    println!("✅ Generated PlantUML format");

    let json_content = visualizer.generate_json_schema()?;
    std::fs::write(format!("{}/schema.json", output_dir), json_content)?;
    println!("✅ Generated JSON schema");

    // Generate export formats
    println!("\n📤 Generating export formats...");
    
    let svg_content = visualizer.export_svg()?;
    std::fs::write(format!("{}/schema.svg", output_dir), svg_content)?;
    println!("✅ Generated SVG export");

    let pdf_content = visualizer.export_pdf()?;
    std::fs::write(format!("{}/schema.pdf", output_dir), pdf_content)?;
    println!("✅ Generated PDF export");

    let visio_content = visualizer.export_visio()?;
    std::fs::write(format!("{}/schema.vsdx", output_dir), visio_content)?;
    println!("✅ Generated Visio export");

    // Demonstrate real-time monitoring
    println!("\n🔍 Setting up real-time schema monitoring...");
    
    let monitoring_config = MonitoringConfig {
        check_interval_seconds: 10,
        enable_notifications: true,
        track_schema_changes: true,
        track_table_changes: true,
        track_column_changes: true,
        track_relationship_changes: true,
    };

    let monitor = SchemaMonitor::new(monitoring_config);
    
    // Start monitoring with a callback
    monitor.start_monitoring("postgres://localhost/test", |event| {
        println!("📡 Schema change detected: {} - {}", 
            event.event_type, event.change_description);
    }).await?;

    println!("✅ Schema monitoring started (checking every 10 seconds)");
    println!("   Press Ctrl+C to stop monitoring...");

    // Simulate some monitoring time
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
    
    // Stop monitoring
    monitor.stop_monitoring();
    println!("✅ Schema monitoring stopped");

    // Show monitoring results
    let change_history = monitor.get_change_history();
    println!("\n📋 Monitoring Results:");
    println!("   Total events captured: {}", change_history.len());
    
    for event in change_history.iter().take(5) {
        println!("   - {}: {} ({})", 
            event.timestamp.format("%H:%M:%S"),
            event.change_description,
            event.severity);
    }

    println!("\n🎉 All visualizations generated successfully!");
    println!("📁 Check the '{}' directory for output files", output_dir);
    println!("\n📋 Generated files:");
    println!("   • schema.dot (Graphviz DOT)");
    println!("   • schema.mmd (Mermaid)");
    println!("   • schema.html (Interactive HTML)");
    println!("   • schema.puml (PlantUML)");
    println!("   • schema.json (JSON Schema)");
    println!("   • schema.svg (SVG Export)");
    println!("   • schema.pdf (PDF Export)");
    println!("   • schema.vsdx (Visio Export)");

    Ok(())
}

fn create_sample_schema() -> Vec<Table> {
    vec![
        Table {
            name: "users".to_string(),
            columns: vec![
                Column {
                    name: "id".to_string(),
                    data_type: "INTEGER".to_string(),
                    is_nullable: false,
                    is_primary_key: true,
                    is_foreign_key: false,
                },
                Column {
                    name: "username".to_string(),
                    data_type: "VARCHAR(50)".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                },
                Column {
                    name: "email".to_string(),
                    data_type: "VARCHAR(100)".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                },
                Column {
                    name: "created_at".to_string(),
                    data_type: "TIMESTAMP".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                },
            ],
        },
        Table {
            name: "posts".to_string(),
            columns: vec![
                Column {
                    name: "id".to_string(),
                    data_type: "INTEGER".to_string(),
                    is_nullable: false,
                    is_primary_key: true,
                    is_foreign_key: false,
                },
                Column {
                    name: "user_id".to_string(),
                    data_type: "INTEGER".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: true,
                },
                Column {
                    name: "title".to_string(),
                    data_type: "VARCHAR(200)".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                },
                Column {
                    name: "content".to_string(),
                    data_type: "TEXT".to_string(),
                    is_nullable: true,
                    is_primary_key: false,
                    is_foreign_key: false,
                },
                Column {
                    name: "published_at".to_string(),
                    data_type: "TIMESTAMP".to_string(),
                    is_nullable: true,
                    is_primary_key: false,
                    is_foreign_key: false,
                },
            ],
        },
        Table {
            name: "comments".to_string(),
            columns: vec![
                Column {
                    name: "id".to_string(),
                    data_type: "INTEGER".to_string(),
                    is_nullable: false,
                    is_primary_key: true,
                    is_foreign_key: false,
                },
                Column {
                    name: "post_id".to_string(),
                    data_type: "INTEGER".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: true,
                },
                Column {
                    name: "user_id".to_string(),
                    data_type: "INTEGER".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: true,
                },
                Column {
                    name: "content".to_string(),
                    data_type: "TEXT".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                },
                Column {
                    name: "created_at".to_string(),
                    data_type: "TIMESTAMP".to_string(),
                    is_nullable: false,
                    is_primary_key: false,
                    is_foreign_key: false,
                },
            ],
        },
    ]
}

fn create_sample_relationships() -> Vec<Relationship> {
    vec![
        Relationship {
            from_table: "posts".to_string(),
            from_column: "user_id".to_string(),
            to_table: "users".to_string(),
            to_column: "id".to_string(),
            relationship_type: "CASCADE:CASCADE".to_string(),
        },
        Relationship {
            from_table: "comments".to_string(),
            from_column: "post_id".to_string(),
            to_table: "posts".to_string(),
            to_column: "id".to_string(),
            relationship_type: "CASCADE:CASCADE".to_string(),
        },
        Relationship {
            from_table: "comments".to_string(),
            from_column: "user_id".to_string(),
            to_table: "users".to_string(),
            to_column: "id".to_string(),
            relationship_type: "CASCADE:CASCADE".to_string(),
        },
    ]
}
