use std::collections::HashMap;
use convert_case::{Case, Casing};
use chrono::NaiveDate;

pub fn generate_header(author: &str, github_link: &str, date: NaiveDate) -> String {
    format!(
        "/*\n * This code was generated by rust_orm_gen.\n * GitHub: {}\n * Date: {}\n * Author: {}\n */\n\n",
        github_link, date.format("%Y-%m-%d"), author
    )
}

pub fn generate_crud_operations(table_name: &str, columns: HashMap<String, String>, author: &str, github_link: &str, date: NaiveDate) -> String {
    let header = generate_header(author, github_link, date);
    let struct_name = table_name.to_case(Case::Pascal);
    let mut crud_ops = format!("{}use tokio_postgres::Client;\nuse crate::query_builder::QueryBuilder;\n\n", header);

    // Sort the column names to ensure consistent order
    let mut column_names: Vec<String> = columns.keys().cloned().collect();
    column_names.sort();

    // Generate Create function
    crud_ops.push_str(&format!(
        "pub async fn create_{table_name}(client: &Client, entity: &{struct_name}) -> Result<{struct_name}, tokio_postgres::Error> {{
    let (query, params) = QueryBuilder::insert::<{struct_name}>()
        .values(&[{}])
        .returning(&[{}])
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok({struct_name} {{
        {}
    }})
}}\n\n",
        column_names.iter().map(|name| format!("&entity.{}", name.replace(" ", "_"))).collect::<Vec<_>>().join(", "),
        column_names.iter().map(|name| format!("\"{}\"", name)).collect::<Vec<_>>().join(", "),
        column_names.iter().map(|name| format!("{}: row.get(\"{}\"),", name.replace(" ", "_"), name)).collect::<Vec<_>>().join("\n        ")
    ));

    // Generate Read function
    crud_ops.push_str(&format!(
        "pub async fn get_{table_name}(client: &Client, id: i32) -> Result<{struct_name}, tokio_postgres::Error> {{
    let (query, params) = QueryBuilder::select::<{struct_name}>()
        .where_clause(\"id = $1\")
        .bind_param(id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok({struct_name} {{
        {}
    }})
}}\n\n",
        column_names.iter().map(|name| format!("{}: row.get(\"{}\"),", name.replace(" ", "_"), name)).collect::<Vec<_>>().join("\n        ")
    ));

    // Generate Update function
    crud_ops.push_str(&format!(
        "pub async fn update_{table_name}(client: &Client, entity: &{struct_name}) -> Result<{struct_name}, tokio_postgres::Error> {{
    let (query, params) = QueryBuilder::update::<{struct_name}>()
        .set_values(&[{}])
        .where_clause(\"id = $1\")
        .bind_param(entity.id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok({struct_name} {{
        {}
    }})
}}\n\n",
        column_names.iter().enumerate().map(|(_i, name)| format!("(\"{}\", &entity.{})", name, name.replace(" ", "_"))).collect::<Vec<_>>().join(", "),
        column_names.iter().map(|name| format!("{}: row.get(\"{}\"),", name.replace(" ", "_"), name)).collect::<Vec<_>>().join("\n        ")
    ));

    // Generate Delete function
    crud_ops.push_str(&format!(
        "pub async fn delete_{table_name}(client: &Client, id: i32) -> Result<bool, tokio_postgres::Error> {{
    let (query, params) = QueryBuilder::delete::<{struct_name}>()
        .where_clause(\"id = $1\")
        .bind_param(id)
        .build();
    
    let result = client.execute(&query, &params[..]).await?;
    
    Ok(result > 0)
}}\n\n"
    ));

    // Generate List function
    crud_ops.push_str(&format!(
        "pub async fn list_{table_name}(client: &Client, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<{struct_name}>, tokio_postgres::Error> {{
    let mut query_builder = QueryBuilder::select::<{struct_name}>();
    
    if let Some(limit_val) = limit {{
        query_builder = query_builder.limit(limit_val as usize);
    }}
    
    if let Some(offset_val) = offset {{
        query_builder = query_builder.offset(offset_val as usize);
    }}
    
    let (query, params) = query_builder.build();
    
    let rows = client.query(&query, &params[..]).await?;
    
    let entities = rows.into_iter().map(|row| {struct_name} {{
        {}
    }}).collect();
    
    Ok(entities)
}}\n",
        column_names.iter().map(|name| format!("{}: row.get(\"{}\"),", name.replace(" ", "_"), name)).collect::<Vec<_>>().join("\n        ")
    ));

    crud_ops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_crud_operations() {
        let mut columns = HashMap::new();
        columns.insert("id".to_string(), "integer".to_string());
        columns.insert("name".to_string(), "text".to_string());
        columns.insert("zip code".to_string(), "text".to_string());

        let fixed_date = NaiveDate::from_ymd_opt(2024, 7, 24).unwrap();
        let result = generate_crud_operations("users", columns, "Tom Blanchard", "https://github.com/tomblanchard312/rust_orm_gen", fixed_date);

        // Basic checks for the presence of all CRUD operations
        assert!(result.contains("pub async fn create_users"));
        assert!(result.contains("pub async fn get_users"));
        assert!(result.contains("pub async fn update_users"));
        assert!(result.contains("pub async fn delete_users"));
        assert!(result.contains("pub async fn list_users"));

        // Check for the use of QueryBuilder
        assert!(result.contains("use crate::query_builder::QueryBuilder;"));
        assert!(result.contains("QueryBuilder::insert"));
        assert!(result.contains("QueryBuilder::select"));
        assert!(result.contains("QueryBuilder::update"));
        assert!(result.contains("QueryBuilder::delete"));

        // Check for proper handling of the "zip code" column
        assert!(result.contains("zip_code: row.get(\"zip code\"),"));

        // Check for the correct use of &params[..]
        assert!(result.contains("client.query_one(&query, &params[..]).await?"));
        assert!(result.contains("client.execute(&query, &params[..]).await?"));
        assert!(result.contains("client.query(&query, &params[..]).await?"));
    }
}