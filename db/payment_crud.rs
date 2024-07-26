/*
 * This code was generated by rust_orm_gen.
 * GitHub: https://github.com/tomblanchard312/rust_orm_gen
 * Date: 2024-07-26
 * Author: Tom Blanchard
 */

use tokio_postgres::Client;
use crate::query_builder::QueryBuilder;

pub async fn create_payment(client: &Client, entity: &Payment) -> Result<Payment, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::insert::<Payment>()
        .values(&[&entity.amount, &entity.customer_id, &entity.payment_date, &entity.payment_id, &entity.rental_id, &entity.staff_id])
        .returning(&["amount", "customer_id", "payment_date", "payment_id", "rental_id", "staff_id"])
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Payment {
        amount: row.get("amount"),
        customer_id: row.get("customer_id"),
        payment_date: row.get("payment_date"),
        payment_id: row.get("payment_id"),
        rental_id: row.get("rental_id"),
        staff_id: row.get("staff_id"),
    })
}

pub async fn get_payment(client: &Client, id: i32) -> Result<Payment, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::select::<Payment>()
        .where_clause("id = $1")
        .bind_param(id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Payment {
        amount: row.get("amount"),
        customer_id: row.get("customer_id"),
        payment_date: row.get("payment_date"),
        payment_id: row.get("payment_id"),
        rental_id: row.get("rental_id"),
        staff_id: row.get("staff_id"),
    })
}

pub async fn update_payment(client: &Client, entity: &Payment) -> Result<Payment, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::update::<Payment>()
        .set_values(&[("amount", &entity.amount), ("customer_id", &entity.customer_id), ("payment_date", &entity.payment_date), ("payment_id", &entity.payment_id), ("rental_id", &entity.rental_id), ("staff_id", &entity.staff_id)])
        .where_clause("id = $1")
        .bind_param(entity.id)
        .build();
    
    let row = client.query_one(&query, &params[..]).await?;
    
    Ok(Payment {
        amount: row.get("amount"),
        customer_id: row.get("customer_id"),
        payment_date: row.get("payment_date"),
        payment_id: row.get("payment_id"),
        rental_id: row.get("rental_id"),
        staff_id: row.get("staff_id"),
    })
}

pub async fn delete_payment(client: &Client, id: i32) -> Result<bool, tokio_postgres::Error> {
    let (query, params) = QueryBuilder::delete::<Payment>()
        .where_clause("id = $1")
        .bind_param(id)
        .build();
    
    let result = client.execute(&query, &params[..]).await?;
    
    Ok(result > 0)
}

pub async fn list_payment(client: &Client, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Payment>, tokio_postgres::Error> {
    let mut query_builder = QueryBuilder::select::<Payment>();
    
    if let Some(limit_val) = limit {
        query_builder = query_builder.limit(limit_val as usize);
    }
    
    if let Some(offset_val) = offset {
        query_builder = query_builder.offset(offset_val as usize);
    }
    
    let (query, params) = query_builder.build();
    
    let rows = client.query(&query, &params[..]).await?;
    
    let entities = rows.into_iter().map(|row| Payment {
        amount: row.get("amount"),
        customer_id: row.get("customer_id"),
        payment_date: row.get("payment_date"),
        payment_id: row.get("payment_id"),
        rental_id: row.get("rental_id"),
        staff_id: row.get("staff_id"),
    }).collect();
    
    Ok(entities)
}