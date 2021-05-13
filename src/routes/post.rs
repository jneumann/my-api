use crate::custom_types::*;
use crate::error::Error::*;
use crate::models::{
    NewPost,
    Post,
};
use crate::schema::posts;
use diesel::ExpressionMethods;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use serde::{
    Deserialize,
};
use warp::Reply;
use warp::reply::json;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
}

pub async fn create(req: CreatePostRequest, db_pool: DBCon) -> WebResult<impl Reply> {
    let insert: std::result::Result<Post, crate::error::Error> = diesel::insert_into(posts::table)
            .values(NewPost {
                title: req.title,
                body: req.body,
            })
            .get_result(&db_pool)
            .map_err(|err| {
                DBQueryError(err)
            });

    Ok(json(&insert.unwrap()))
}

pub async fn read(db_pool: DBCon) -> WebResult<impl Reply> {
    use crate::schema::posts::dsl::*;

    let post_query = posts
        .limit(5)
        .load::<Post>(&db_pool)
        .map_err(|err| {
            DBQueryError(err)
        });

    Ok(json(&post_query.unwrap()))
}

pub async fn update(
    post_id: i32,
    req: CreatePostRequest,
    db_pool: DBCon) -> WebResult<impl Reply> {
    use crate::schema::posts::dsl::*;

    let update: std::result::Result<Post, crate::error::Error> = diesel::update(posts.find(post_id))
            .set((
                title.eq(req.title),
                body.eq(req.body)
            ))
            .get_result(&db_pool)
            .map_err(|err| {
                DBQueryError(err)
            });

    Ok(json(&update.unwrap()))
}

pub async fn delete( post_id: i32, db_pool: DBCon) -> WebResult<impl Reply> {
    use crate::schema::posts::dsl::*;

    let response = diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&db_pool)
        .map_err(|err| {
            DBQueryError(err)
        });

    //match response {
    //    Ok(o) => Ok(json(&o)),
    //    Err(e) => Ok(json(&e))
    //}

    Ok(json(&response.unwrap()))
}

//pub async fn create_todo_handler(_: String, body: TodoRequest, db_pool: DBPool) -> WebResult<impl Reply> {
//    Ok(json(&TodoResponse::of(
//                todo::create_todo(&db_pool, body)
//                .await
//                .map_err(|e| reject::custom(e))?,
//    )))
//}
//
//#[derive(Deserialize)]
//pub struct SearchQuery {
//    search: Option<String>,
//}
//
//pub async fn list_todos_handler(_: String, query: SearchQuery, db_pool: DBPool) -> WebResult<impl Reply> {
//    let todos = todo::fetch_todos(&db_pool, query.search)
//            .await
//            .map_err(|e| reject::custom(e))?;
//    Ok(json::<Vec<_>>(
//            &todos.into_iter().map(|t| TodoResponse::of(t)).collect(),
//    ))
//}
//
//pub async fn update_todo_handler(_: String,
//    id: i32,
//    body: TodoUpdateRequest,
//    db_pool: DBPool,
//) -> WebResult<impl Reply> {
//    Ok(json(&TodoResponse::of(
//        todo::update_todo(&db_pool, id, body)
//            .await
//            .map_err(|e| reject::custom(e))?,
//    )))
//}
//
//pub async fn delete_todo_handler(_: String, id: i32, db_pool: DBPool) -> WebResult<impl Reply> {
//    todo::delete_todo(&db_pool, id)
//            .await
//            .map_err(|e| reject::custom(e))?;
//    Ok(StatusCode::OK)
//}
