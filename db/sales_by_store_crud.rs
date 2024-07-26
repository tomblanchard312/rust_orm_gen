/*
 * This code was generated by rust_orm_gen.
 * GitHub: https://github.com/tomblanchard312/rust_orm_gen
 * Date: 2024-07-26
 * Author: Tom Blanchard
 */

use tokio_postgres::Client;
use crate::query_builder::QueryBuilder;

pub async fn create_sales_by_store(client: &Client, entity: &SalesByStore) -> Result<SalesByStore, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::insert::<SalesByStore>()
        .values(&[&entity.manager, &entity.store, &entity.total_sales])
        .returning(&["manager", "store", "total_sales"])
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(SalesByStore {
        manager: row.get("manager"),
        store: row.get("store"),
        total_sales: row.get("total_sales"),
    })
}

pub async fn get_sales_by_store(client: &Client, id: i32) -> Result<SalesByStore, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::select::<SalesByStore>()
        .where_clause("id = $1")
        .bind_param(id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(SalesByStore {
        manager: row.get("manager"),
        store: row.get("store"),
        total_sales: row.get("total_sales"),
    })
}

pub async fn update_sales_by_store(client: &Client, entity: &SalesByStore) -> Result<SalesByStore, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::update::<SalesByStore>()
        .set_values(&[("manager", &entity.manager), ("store", &entity.store), ("total_sales", &entity.total_sales)])
        .where_clause("id = $1")
        .bind_param(entity.id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(SalesByStore {
        manager: row.get("manager"),
        store: row.get("store"),
        total_sales: row.get("total_sales"),
    })
}

pub async fn delete_sales_by_store(client: &Client, id: i32) -> Result<bool, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::delete::<SalesByStore>()
        .where_clause("id = $1")
        .bind_param(id)
        .build();
    
    let result = client.execute(&query, &params[..]).await?;
    
    Ok(result > 0)
}

pub async fn list_sales_by_store(client: &Client, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<SalesByStore>, tokio_postgres::Error> {
    let mut query_builder = QueryBuilder::select::<SalesByStore>();
    
    if let Some(limit_val) = limit {
        query_builder = query_builder.limit(limit_val as usize);
    }
    
    if let Some(offset_val) = offset {
        query_builder = query_builder.offset(offset_val as usize);
    }
    
    let (query, params) = query_builder.build();
    
    let rows = client.query(&query, &params[..]).await?;
    
    let entities = rows.into_iter().map(|row| SalesByStore {
        manager: row.get("manager"),
        store: row.get("store"),
        total_sales: row.get("total_sales"),
    }).collect();
    
    Ok(entities)
}
