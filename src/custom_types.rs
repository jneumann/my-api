use crate::error;
use std::collections::HashMap;
use std::sync::Arc;
use warp::{ Rejection};

#[derive(Clone)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

pub type Result<T> = std::result::Result<T, error::Error>;
pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Users = Arc<HashMap<String, User>>;
