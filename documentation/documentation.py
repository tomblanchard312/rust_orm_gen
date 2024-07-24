from fpdf import FPDF

class PDF(FPDF):
    def header(self):
        self.set_font('Arial', 'B', 12)
        self.cell(0, 10, 'rust_orm_gen Documentation', 0, 1, 'C')

    def chapter_title(self, chapter_title):
        self.set_font('Arial', 'B', 12)
        self.cell(0, 10, chapter_title, 0, 1, 'L')
        self.ln(5)

    def chapter_body(self, body):
        self.set_font('Arial', '', 12)
        self.multi_cell(0, 10, body)
        self.ln()

pdf = PDF()
pdf.add_page()

# Introduction
pdf.chapter_title('1. Introduction')
pdf.chapter_body('rust_orm_gen is a Rust library designed to reverse engineer PostgreSQL databases and automatically generate Rust structs and CRUD operations. This tool simplifies the process of interacting with a PostgreSQL database in Rust, ensuring that your code is clean, maintainable, and efficient.')

# Installation
pdf.chapter_title('2. Installation')
pdf.chapter_body('Add rust_orm_gen to your Cargo.toml:\n\n[dependencies]\nrust_orm_gen = { path = "../path_to_your_local_crate" }\ntokio = { version = "1", features = ["full"] }\ndotenv = "0.15.0"')

# Configuration
pdf.chapter_title('3. Configuration')
pdf.chapter_body('Ensure your .env file is correctly configured with the database URL:\n\nDATABASE_URL=postgres://user:password@localhost/mydb\n\nReplace user, password, and mydb with your actual PostgreSQL credentials and database name.')

# Usage
pdf.chapter_title('4. Usage')
pdf.chapter_body('Step 1: Initialize the Database Context\n\nCreate a file named main.rs to run the reverse engineering tool:\n\nmod context;\nmod metadata;\nmod generator;\nmod crud;\nmod db;\n\nuse crate::context::DbContext;\nuse dotenv::dotenv;\nuse std::env;\n\n#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {\n    dotenv().ok();\n    let database_url = env::var("DATABASE_URL")?;\n    let db_context = DbContext::new(&database_url).await?;\n\n    let output_dir = "db";\n\n    db_context.reverse_engineer(output_dir).await?;\n\n    Ok(())\n}\n\nStep 2: Run the Program\n\nIn your terminal, navigate to your project directory and run:\n\ncargo run')

# Example Project
pdf.chapter_title('5. Example Project')
pdf.chapter_body('Here\'s a step-by-step example of how to use rust_orm_gen in your own project.\n\n1. Create a New Project\n\ncargo new my_project\ncd my_project\n\n2. Add Dependencies\n\nUpdate the Cargo.toml file in your project:\n\n[dependencies]\nrust_orm_gen = { path = "../path_to_your_local_crate" }\ntokio = { version = "1", features = ["full"] }\ndotenv = "0.15.0"\n\n3. Create .env File\n\nAdd your PostgreSQL connection string to a .env file:\n\nDATABASE_URL=postgres://user:password@localhost/mydb\n\n4. Set Up Main Function\n\nCreate a main.rs file in the src directory:\n\nmod context;\nmod metadata;\nmod generator;\nmod crud;\nmod db;\n\nuse crate::context::DbContext;\nuse dotenv::dotenv;\nuse std::env;\n\n#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {\n    dotenv().ok();\n    let database_url = env::var("DATABASE_URL")?;\n    let db_context = DbContext::new(&database_url).await?;\n\n    let output_dir = "db";\n\n    db_context.reverse_engineer(output_dir).await?;\n\n    Ok(())\n}\n\n5. Run the Program\n\ncargo run')

# Generated Code Structure
pdf.chapter_title('6. Generated Code Structure')
pdf.chapter_body('After running the program, the generated ORM files will be saved in the db directory. For example, if you have a table named users, it will generate two files: users.rs and users_crud.rs.\n\nusers.rs\n\n#[derive(Debug, Serialize, Deserialize)]\npub struct Users {\n    #[serde(rename = "id")] pub id: i32,\n    #[serde(rename = "first name")] pub first_name: String,\n    #[serde(rename = "last name")] pub last_name: String,\n}\n\nusers_crud.rs\n\nuse tokio_postgres::Client;\n\npub async fn create_users(client: &Client, entity: &Users) -> Result<Users, tokio_postgres::Error> {\n    let row = client.query_one(\n        "INSERT INTO users (id, \"first name\", \"last name\") VALUES ($1, $2, $3) RETURNING *",\n        &[&entity.id, &entity.first_name, &entity.last_name]\n    ).await?;\n\n    Ok(Users {\n        id: row.get("id"),\n        first_name: row.get("first name"),\n        last_name: row.get("last name"),\n    })\n}\n\npub async fn get_users(client: &Client, id: i32) -> Result<Users, tokio_postgres::Error> {\n    let row = client.query_one(\n        "SELECT * FROM users WHERE id = $1",\n        &[&id]\n    ).await?;\n\n    Ok(Users {\n        id: row.get("id"),\n        first_name: row.get("first name"),\n        last_name: row.get("last name"),\n    })\n}\n\npub async fn update_users(client: &Client, entity: &Users) -> Result<Users, tokio_postgres::Error> {\n    let row = client.query_one(\n        "UPDATE users SET \"first name\" = $1, \"last name\" = $2 WHERE id = $3 RETURNING *",\n        &[&entity.first_name, &entity.last_name, &entity.id]\n    ).await?;\n\n    Ok(Users {\n        id: row.get("id"),\n        first_name: row.get("first name"),\n        last_name: row.get("last name"),\n    })\n}\n\npub async fn delete_users(client: &Client, id: i32) -> Result<u64, tokio_postgres::Error> {\n    let result = client.execute(\n        "DELETE FROM users WHERE id = $1",\n        &[&id]\n    ).await?;\n\n    Ok(result)\n}')

# Integrating the Generated Code
pdf.chapter_title('7. Integrating the Generated Code')
pdf.chapter_body('To use the generated ORM code in your project:\n\n1. Include the Generated Files\n\nIn your main project file (e.g., main.rs):\n\nmod db {\n    pub mod users;\n    pub mod users_crud;\n}\n\n2. Use the Generated Code\n\nUse the generated code to interact with the database:\n\nuse db::users::Users;\nuse db::users_crud::{create_users, get_users, update_users, delete_users};\nuse tokio_postgres::Client;\n\n#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {\n    dotenv().ok();\n    let database_url = env::var("DATABASE_URL")?;\n    let (client, connection) = tokio_postgres::connect(database_url, tokio_postgres::NoTls).await?;\n\n    tokio::spawn(async move {\n        if let Err(e) = connection.await {\n            eprintln!("connection error: {}", e);\n        }\n    });\n\n    let new_user = Users {\n        id: 1,\n        first_name: "John".to_string(),\n        last_name: "Doe".to_string(),\n    };\n\n    let created_user = create_users(&client, &new_user).await?;\n    println!("Created user: {:?}", created_user);\n\n    let fetched_user = get_users(&client, 1).await?;\n    println!("Fetched user: {:?}", fetched_user);\n\n    let updated_user = Users {\n        id: 1,\n        first_name: "Jane".to_string(),\n        last_name: "Doe".to_string(),\n    };\n    let updated_user = update_users(&client, &updated_user).await?;\n    println!("Updated user: {:?}", updated_user);\n\n    let rows_deleted = delete_users(&client, 1).await?;\n    println!("Deleted {} user(s)", rows_deleted);\n\n    Ok(())\n}')

pdf.output('rust_orm_gen_documentation.pdf')
