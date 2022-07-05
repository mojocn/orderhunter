


use serde::Serialize;
use serde::Deserialize;



use sqlx::{Result};
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Debug, Serialize,Deserialize)]
pub struct User {
    id: i32,
    name:String,
    email:String,
}


impl User {

    pub async  fn list(db :&SqlitePool)->Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM users ORDER BY ?").bind("id").fetch_all(db).await
    }


}