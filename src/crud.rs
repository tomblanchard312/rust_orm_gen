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
    let mut crud_ops = format!("{}use tokio_postgres::Client;\n\n", header);

    // Sort the column names to ensure consistent order
    let mut column_names: Vec<String> = columns.keys().cloned().collect();
    column_names.sort();

    // Generate Create function
    let column_placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("${}", i)).collect();
    let insert_query = format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
        table_name,
        column_names.join(", "),
        column_placeholders.join(", ")
    );
    crud_ops.push_str(&format!(
        "pub async fn create_{table_name}(client: &Client, entity: &{struct_name}) -> Result<{struct_name}, tokio_postgres::Error> {{
    let row = client.query_one(
        \"{}\",
        &[{}]
    ).await?;
    
    Ok({struct_name} {{
        {}
    }})
}}\n\n",
        insert_query,
        column_names.iter().map(|name| format!("&entity.{}", name.replace(" ", "_"))).collect::<Vec<_>>().join(", "),
        column_names.iter().map(|name| format!("{}: row.get(\"{}\"),", name.replace(" ", "_"), name)).collect::<Vec<_>>().join("\n        ")
    ));

    // Generate Read function
    let select_query = format!(
        "SELECT * FROM {} WHERE id = $1",
        table_name
    );
    crud_ops.push_str(&format!(
        "pub async fn get_{table_name}(client: &Client, id: i32) -> Result<{struct_name}, tokio_postgres::Error> {{
    let row = client.query_one(
        \"{}\",
        &[&id]
    ).await?;
    
    Ok({struct_name} {{
        {}
    }})
}}\n\n",
        select_query,
        column_names.iter().map(|name| format!("{}: row.get(\"{}\"),", name.replace(" ", "_"), name)).collect::<Vec<_>>().join("\n        ")
    ));

    // Generate Update function
    let update_set: Vec<String> = column_names.iter().map(|name| format!("{} = ${}", name, column_names.iter().position(|x| x == name).unwrap() + 1)).collect();
    let update_query = format!(
        "UPDATE {} SET {} WHERE id = $1 RETURNING *",
        table_name,
        update_set.join(", ")
    );
    crud_ops.push_str(&format!(
        "pub async fn update_{table_name}(client: &Client, entity: &{struct_name}) -> Result<{struct_name}, tokio_postgres::Error> {{
    let row = client.query_one(
        \"{}\",
        &[{}]
    ).await?;
    
    Ok({struct_name} {{
        {}
    }})
}}\n\n",
        update_query,
        column_names.iter().map(|name| format!("&entity.{}", name.replace(" ", "_"))).collect::<Vec<_>>().join(", "),
        column_names.iter().map(|name| format!("{}: row.get(\"{}\"),", name.replace(" ", "_"), name)).collect::<Vec<_>>().join("\n        ")
    ));

    // Generate Delete function
    let delete_query = format!(
        "DELETE FROM {} WHERE id = $1",
        table_name
    );
    crud_ops.push_str(&format!(
        "pub async fn delete_{table_name}(client: &Client, id: i32) -> Result<u64, tokio_postgres::Error> {{
    let result = client.execute(
        \"{}\",
        &[&id]
    ).await?;
    
    Ok(result)
}}\n",
        delete_query
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

        let expected = r#"/*
 * This code was generated by rust_orm_gen.
 * GitHub: https://github.com/tomblanchard312/rust_orm_gen
 * Date: 2024-07-24
 * Author: Tom Blanchard
 */

use tokio_postgres::Client;

pub async fn create_users(client: &Client, entity: &Users) -> Result<Users, tokio_postgres::Error> {
    let row = client.query_one(
        "INSERT INTO users (id, name, zip code) VALUES ($1, $2, $3) RETURNING *",
        &[&entity.id, &entity.name, &entity.zip_code]
    ).await?;
    
    Ok(Users {
        id: row.get("id"),
        name: row.get("name"),
        zip_code: row.get("zip code"),
    })
}

pub async fn get_users(client: &Client, id: i32) -> Result<Users, tokio_postgres::Error> {
    let row = client.query_one(
        "SELECT * FROM users WHERE id = $1",
        &[&id]
    ).await?;
    
    Ok(Users {
        id: row.get("id"),
        name: row.get("name"),
        zip_code: row.get("zip code"),
    })
}

pub async fn update_users(client: &Client, entity: &Users) -> Result<Users, tokio_postgres::Error> {
    let row = client.query_one(
        "UPDATE users SET id = $1, name = $2, zip code = $3 WHERE id = $1 RETURNING *",
        &[&entity.id, &entity.name, &entity.zip_code]
    ).await?;
    
    Ok(Users {
        id: row.get("id"),
        name: row.get("name"),
        zip_code: row.get("zip code"),
    })
}

pub async fn delete_users(client: &Client, id: i32) -> Result<u64, tokio_postgres::Error> {
    let result = client.execute(
        "DELETE FROM users WHERE id = $1",
        &[&id]
    ).await?;
    
    Ok(result)
}
"#;

        assert_eq!(result.trim(), expected.trim());
    }
}