
use axum::{
    routing::{get, post},
    extract::Extension,
    Router,
    response::Json,
};

use super::{ApiContext, ResultObject};
use crate::models::{self, User};


async fn list(
    Extension(ctx): Extension<ApiContext>,
) -> Json<ResultObject<Vec<User>>> {
    let users = models::User::list(&ctx.db).await.unwrap();
    Json(ResultObject{
        code: 200,
        msg: "success".to_string(),
        data: users,
    })

}


pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/users", get(list))
}