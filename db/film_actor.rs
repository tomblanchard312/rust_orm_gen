/*
 * This code was generated by rust_orm_gen.
 * GitHub: https://github.com/tomblanchard312/rust_orm_gen
 * Date: 2024-07-26
 * Author: Tom Blanchard
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct FilmActor {
    #[serde(rename = "actor_id")] pub actor_id: i16,
    #[serde(rename = "film_id")] pub film_id: i16,
    #[serde(rename = "last_update")] pub last_update: String,
}
