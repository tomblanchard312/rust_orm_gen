/*
 * This code was generated by rust_orm_gen.
 * GitHub: https://github.com/tomblanchard312/rust_orm_gen
 * Date: 2024-07-26
 * Author: Tom Blanchard
 */

use tokio_postgres::Client;
use crate::query_builder::QueryBuilder;

pub async fn create_address(client: &Client, entity: &Address) -> Result<Address, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::insert::<Address>()
        .values(&[&entity.address, &entity.address2, &entity.address_id, &entity.city_id, &entity.district, &entity.last_update, &entity.phone, &entity.postal_code])
        .returning(&["address", "address2", "address_id", "city_id", "district", "last_update", "phone", "postal_code"])
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Address {
        address: row.get("address"),
        address2: row.get("address2"),
        address_id: row.get("address_id"),
        city_id: row.get("city_id"),
        district: row.get("district"),
        last_update: row.get("last_update"),
        phone: row.get("phone"),
        postal_code: row.get("postal_code"),
    })
}

pub async fn get_address(client: &Client, id: i32) -> Result<Address, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::select::<Address>()
        .where_clause("id = $1")
        .bind_param(id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Address {
        address: row.get("address"),
        address2: row.get("address2"),
        address_id: row.get("address_id"),
        city_id: row.get("city_id"),
        district: row.get("district"),
        last_update: row.get("last_update"),
        phone: row.get("phone"),
        postal_code: row.get("postal_code"),
    })
}

pub async fn update_address(client: &Client, entity: &Address) -> Result<Address, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::update::<Address>()
        .set_values(&[("address", &entity.address), ("address2", &entity.address2), ("address_id", &entity.address_id), ("city_id", &entity.city_id), ("district", &entity.district), ("last_update", &entity.last_update), ("phone", &entity.phone), ("postal_code", &entity.postal_code)])
        .where_clause("id = $1")
        .bind_param(entity.id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Address {
        address: row.get("address"),
        address2: row.get("address2"),
        address_id: row.get("address_id"),
        city_id: row.get("city_id"),
        district: row.get("district"),
        last_update: row.get("last_update"),
        phone: row.get("phone"),
        postal_code: row.get("postal_code"),
    })
}

pub async fn delete_address(client: &Client, id: i32) -> Result<bool, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::delete::<Address>()
        .where_clause("id = $1")
        .bind_param(id)
        .build();
    
    let result = client.execute(&query, &params[..]).await?;
    
    Ok(result > 0)
}

pub async fn list_address(client: &Client, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Address>, tokio_postgres::Error> {
    let mut query_builder = QueryBuilder::select::<Address>();
    
    if let Some(limit_val) = limit {
        query_builder = query_builder.limit(limit_val as usize);
    }
    
    if let Some(offset_val) = offset {
        query_builder = query_builder.offset(offset_val as usize);
    }
    
    let (query, params) = query_builder.build();
    
    let rows = client.query(&query, &params[..]).await?;
    
    let entities = rows.into_iter().map(|row| Address {
        address: row.get("address"),
        address2: row.get("address2"),
        address_id: row.get("address_id"),
        city_id: row.get("city_id"),
        district: row.get("district"),
        last_update: row.get("last_update"),
        phone: row.get("phone"),
        postal_code: row.get("postal_code"),
    }).collect();
    
    Ok(entities)
}