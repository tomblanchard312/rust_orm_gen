/*
 * This code was generated by rust_orm_gen.
 * GitHub: https://github.com/tomblanchard312/rust_orm_gen
 * Date: 2024-07-26
 * Author: Tom Blanchard
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffList {
    #[serde(rename = "address")] pub address: String,
    #[serde(rename = "city")] pub city: String,
    #[serde(rename = "country")] pub country: String,
    #[serde(rename = "id")] pub id: i32,
    #[serde(rename = "name")] pub name: String,
    #[serde(rename = "phone")] pub phone: String,
    #[serde(rename = "sid")] pub sid: i16,
    #[serde(rename = "zip code")] pub zip_code: String,
}
