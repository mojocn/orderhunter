



use sqlx::{Result};
use sqlx::mysql::MySqlPool;

pub struct Ticket {
    id: String,
    created_at:String,
}


impl Ticket {

    // pub async  fn list(db :&MySqlPool)->Result<Vec<Self>> {
    //     sqlx::query_as::<_, Self>("SELECT * FROM p_work_order_info ORDER BY ?").bind("id").fetch_all(db).await
    // }


}