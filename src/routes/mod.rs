use crate::custom_types::*;
use crate::error::Error::*;
use crate::models::*;
use diesel::ExpressionMethods;
use diesel::prelude::*;
use warp::{
    http::StatusCode,
    Reply,
    Rejection,
};

//pub mod authentication;
//pub mod todo;
pub mod post;

pub async fn health_handler(db_pool: DBCon) -> std::result::Result<impl Reply, Rejection> {
    use crate::schema::posts::dsl::*;

    let _ = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&db_pool)
        .map_err(|err| {
            DBQueryError(err)
        });

    Ok(StatusCode::OK)
}
