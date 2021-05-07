use crate::error::Error;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use mobc::{
    Connection,
    Pool
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio_postgres::NoTls;
use warp::Rejection;

#[derive(Clone)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Users = Arc<HashMap<String, User>>;
pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;
