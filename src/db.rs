use anyhow::{Context, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::env;

pub const DEFAULT_MAX_CONNECTIONS: u32 = 2;

pub fn from_env() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let url = env::var("DATABASE_URL").context("Missing `DATABASE_URL` environment variable")?;
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .max_size(DEFAULT_MAX_CONNECTIONS)
        .build(manager)
        .context("Error creating connection pool")?;
    Ok(pool)
}
