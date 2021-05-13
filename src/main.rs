#[macro_use]
extern crate diesel;

mod auth;
mod custom_types;
mod data;
mod db;
mod error;
mod handler;
mod routes;
mod schema;
mod models;

//use crate::auth::*;
//use crate::custom_types::*;
use crate::db::setup::*;
//use std::collections::HashMap;
//use std::convert::Infallible;
//use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    let connection = establish_connection();

    //let users = Arc::new(init_users());

    let health_route = warp::path!("health")
        .and(with_db(connection.clone()))
        .and_then(routes::health_handler);

    let post = warp::path("post");
    let post_routes = post
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(connection.clone()))
        .and_then(routes::post::create)
        .or(
            post
                .and(warp::get())
                .and(with_db(connection.clone()))
                .and_then(routes::post::read)
        )
        .or(
            post
                .and(warp::put())
                .and(warp::path::param())
                .and(warp::body::json())
                .and(with_db(connection.clone()))
                .and_then(routes::post::update)
        )
        .or(
            post
                .and(warp::delete())
                .and(warp::path::param())
                .and(with_db(connection.clone()))
                .and_then(routes::post::delete)
        );

    //let login_route = warp::path!("login")
    //    .and(warp::post())
    //    .and(with_users(users.clone()))
    //    .and(warp::body::json())
    //    .and_then(routes::authentication::login_handler);

    //let user_route = warp::path!("user")
    //    .and(with_auth(auth::Role::User))
    //    .and_then(routes::authentication::user_handler);

    //let admin_route = warp::path!("admin")
    //    .and(with_auth(auth::Role::Admin))
    //    .and_then(routes::authentication::admin_handler);

    //let todo = warp::path("todo");
    //let todo_routes = todo
    //    .and(warp::get())
    //    .and(with_auth(auth::Role::Admin))
    //    .and(warp::query())
    //    .and(with_db(db_pool.clone()))
    //    .and_then(handler::list_todos_handler)
    //    .or(todo
    //        .and(warp::post())
    //        .and(with_auth(auth::Role::User))
    //        .and(warp::body::json())
    //        .and(with_db(db_pool.clone()))
    //        .and_then(handler::create_todo_handler))
    //    .or(todo
    //        .and(warp::put())
    //        .and(with_auth(auth::Role::User))
    //        .and(warp::path::param())
    //        .and(warp::body::json())
    //        .and(with_db(db_pool.clone()))
    //        .and_then(handler::update_todo_handler))
    //    .or(todo
    //        .and(warp::delete())
    //        .and(with_auth(auth::Role::User))
    //        .and(warp::path::param())
    //        .and(with_db(db_pool.clone()))
    //        .and_then(handler::delete_todo_handler));

    let routes = health_route
        .or(post_routes)
    //    .or(todo_routes)
    //    .or(user_route)
    //    .or(admin_route)
    //    .or(login_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

//fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
//    warp::any().map(move || users.clone())
//}
//
//fn init_users() -> HashMap<String, User> {
//    let mut map = HashMap::new();
//    map.insert(
//        String::from("1"),
//        User {
//            uid: String::from("1"),
//            email: String::from("user@userland.com"),
//            pw: String::from("1234"),
//            role: String::from("User"),
//        },
//    );
//    map.insert(
//        String::from("2"),
//        User {
//            uid: String::from("2"),
//            email: String::from("admin@adminaty.com"),
//            pw: String::from("4321"),
//            role: String::from("Admin"),
//        },
//    );
//    map
//}

