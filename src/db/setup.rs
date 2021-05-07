use std::time::Duration;
use crate::custom_types::*;
use crate::error::Error::*;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use mobc::Pool;
use std::fs;
use std::str::FromStr;
use tokio_postgres::{Config, Error, NoTls};
use warp::Filter;
use std::convert::Infallible;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str("postgres://root:password@127.0.0.1:5432/api")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
            .max_open(DB_POOL_MAX_OPEN)
            .max_idle(DB_POOL_MAX_IDLE)
            .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
            .build(manager))
}

const INIT_SQL: &str = "./db.sql";

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con: DBCon = get_db_con(db_pool).await?;
    con
            .batch_execute(init_file.as_str())
            .await
            .map_err(DBInitError)?;
    Ok(())
}

pub fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
