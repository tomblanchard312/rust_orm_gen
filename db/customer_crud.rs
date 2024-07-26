/*
 * This code was generated by rust_orm_gen.
 * GitHub: https://github.com/tomblanchard312/rust_orm_gen
 * Date: 2024-07-26
 * Author: Tom Blanchard
 */

use tokio_postgres::Client;
use crate::query_builder::QueryBuilder;

pub async fn create_customer(client: &Client, entity: &Customer) -> Result<Customer, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::insert::<Customer>()
        .values(&[&entity.active, &entity.activebool, &entity.address_id, &entity.create_date, &entity.customer_id, &entity.email, &entity.first_name, &entity.last_name, &entity.last_update, &entity.store_id])
        .returning(&["active", "activebool", "address_id", "create_date", "customer_id", "email", "first_name", "last_name", "last_update", "store_id"])
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Customer {
        active: row.get("active"),
        activebool: row.get("activebool"),
        address_id: row.get("address_id"),
        create_date: row.get("create_date"),
        customer_id: row.get("customer_id"),
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        last_update: row.get("last_update"),
        store_id: row.get("store_id"),
    })
}

pub async fn get_customer(client: &Client, id: i32) -> Result<Customer, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::select::<Customer>()
        .where_clause("id = $1")
        .bind_param(id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Customer {
        active: row.get("active"),
        activebool: row.get("activebool"),
        address_id: row.get("address_id"),
        create_date: row.get("create_date"),
        customer_id: row.get("customer_id"),
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        last_update: row.get("last_update"),
        store_id: row.get("store_id"),
    })
}

pub async fn update_customer(client: &Client, entity: &Customer) -> Result<Customer, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::update::<Customer>()
        .set_values(&[("active", &entity.active), ("activebool", &entity.activebool), ("address_id", &entity.address_id), ("create_date", &entity.create_date), ("customer_id", &entity.customer_id), ("email", &entity.email), ("first_name", &entity.first_name), ("last_name", &entity.last_name), ("last_update", &entity.last_update), ("store_id", &entity.store_id)])
        .where_clause("id = $1")
        .bind_param(entity.id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Customer {
        active: row.get("active"),
        activebool: row.get("activebool"),
        address_id: row.get("address_id"),
        create_date: row.get("create_date"),
        customer_id: row.get("customer_id"),
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        last_update: row.get("last_update"),
        store_id: row.get("store_id"),
    })
}

pub async fn delete_customer(client: &Client, id: i32) -> Result<bool, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::delete::<Customer>()
        .where_clause("id = $1")
        .bind_param(id)
        .build();
    
    let result = client.execute(&query, &params[..]).await?;
    
    Ok(result > 0)
}

pub async fn list_customer(client: &Client, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Customer>, tokio_postgres::Error> {
    let mut query_builder = QueryBuilder::select::<Customer>();
    
    if let Some(limit_val) = limit {
        query_builder = query_builder.limit(limit_val as usize);
    }
    
    if let Some(offset_val) = offset {
        query_builder = query_builder.offset(offset_val as usize);
    }
    
    let (query, params) = query_builder.build();
    
    let rows = client.query(&query, &params[..]).await?;
    
    let entities = rows.into_iter().map(|row| Customer {
        active: row.get("active"),
        activebool: row.get("activebool"),
        address_id: row.get("address_id"),
        create_date: row.get("create_date"),
        customer_id: row.get("customer_id"),
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        last_update: row.get("last_update"),
        store_id: row.get("store_id"),
    }).collect();
    
    Ok(entities)
}
