//use std::time::Duration;
use crate::custom_types::*;
use crate::error::Error::*;
//use diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{
    Pool,
    ConnectionManager,
};
use warp::{
    Filter,
    reject,
};

//const DB_POOL_MAX_OPEN: u64 = 32;
//const DB_POOL_MAX_IDLE: u64 = 8;
//const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub fn create_pool() -> DBPool {
    let database_url =  "postgres://root:password@127.0.0.1:5432/api";
    init_pool(&database_url).expect("Failed to create pool")
}

fn init_pool(database_url: &str) -> Result<DBPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .map_err(|e| DBInitError(e))
}

pub fn establish_connection() -> DBPool {
    init_pool("postgres://root:password@127.0.0.1:5432/api").expect("Failed to create pool")
}

pub fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBCon,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db_pool.clone())
        .and_then(|pool: DBPool| async move {
            match pool.get() {
                Ok(conn) => Ok(conn),
                Err(err) => Err(reject::custom(
                        DBPoolError(err)
                ),
            )}
        })
}
