use crate::error::Error;
use diesel::pg::PgConnection;
use diesel::r2d2::{
    Pool,
    PooledConnection,
    ConnectionManager,
    //PoolError
};
//use std::sync::Arc;
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
//pub type Users = Arc<HashMap<String, User>>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBCon = PooledConnection<ConnectionManager<PgConnection>>;
