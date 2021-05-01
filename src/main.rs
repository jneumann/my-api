use auth::{with_auth};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{ Filter};

mod auth;
mod error;
mod routes;
mod custom_types;

use custom_types::*;

#[tokio::main]
async fn main() {
    let users = Arc::new(init_users());

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(routes::authentication::login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(auth::Role::User))
        .and_then(routes::authentication::user_handler);

    let admin_route = warp::path!("admin")
        .and(with_auth(auth::Role::Admin))
        .and_then(routes::authentication::admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@userland.com"),
            pw: String::from("1234"),
            role: String::from("User"),
        },
    );
    map.insert(
        String::from("2"),
        User {
            uid: String::from("2"),
            email: String::from("admin@adminaty.com"),
            pw: String::from("4321"),
            role: String::from("Admin"),
        },
    );
    map
}

