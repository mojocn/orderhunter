

use sqlx::{Result};
use sqlx::mysql::MySqlPool;

pub struct Order {
    id: String,
    created_at:String,
    flow_id:String,
    status:String,
    current_node_id:String,
}


impl Order {

    pub async  fn list(db :&MySqlPool)->Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM orders ORDER BY ?").bind("id").fetch_all(db).await
    }


}