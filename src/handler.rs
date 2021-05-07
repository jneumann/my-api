use crate::custom_types::*;
use crate::data::*;
use crate::db::setup;
use crate::db::todo;
use serde::Deserialize;
use warp::{
    http::StatusCode,
    Reply,
    Rejection,
    reject,
    reply::json,
};

pub async fn health_handler(db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let db = setup::get_db_con(&db_pool)
            .await
            .map_err(|e| reject::custom(e))?;

    db.execute("SELECT 1", &[])
            .await
            .map_err(|e| reject::custom(crate::error::Error::DBQueryError(e)))?;

    Ok(StatusCode::OK)
}

pub async fn create_todo_handler(_: String, body: TodoRequest, db_pool: DBPool) -> WebResult<impl Reply> {
    Ok(json(&TodoResponse::of(
                todo::create_todo(&db_pool, body)
                .await
                .map_err(|e| reject::custom(e))?,
    )))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn list_todos_handler(_: String, query: SearchQuery, db_pool: DBPool) -> WebResult<impl Reply> {
    let todos = todo::fetch_todos(&db_pool, query.search)
            .await
            .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
            &todos.into_iter().map(|t| TodoResponse::of(t)).collect(),
    ))
}

pub async fn update_todo_handler(_: String,
    id: i32,
    body: TodoUpdateRequest,
    db_pool: DBPool,
) -> WebResult<impl Reply> {
    Ok(json(&TodoResponse::of(
        todo::update_todo(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_todo_handler(_: String, id: i32, db_pool: DBPool) -> WebResult<impl Reply> {
    todo::delete_todo(&db_pool, id)
            .await
            .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
