/*
 * This code was generated by rust_orm_gen.
 * GitHub: https://github.com/tomblanchard312/rust_orm_gen
 * Date: 2024-07-26
 * Author: Tom Blanchard
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "category_id")] pub category_id: i32,
    #[serde(rename = "last_update")] pub last_update: String,
    #[serde(rename = "name")] pub name: String,
}