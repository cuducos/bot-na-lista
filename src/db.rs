use anyhow::{Context, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::{env, time::Duration};

pub const DEFAULT_MAX_CONNECTIONS: u32 = 2;

pub fn max_connections() -> u32 {
    if let Ok(txt) = env::var("MAX_CONNECTIONS") {
        if let Ok(num) = txt.parse() {
            return num;
        }
    }
    DEFAULT_MAX_CONNECTIONS
}

pub fn from_env() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let url = env::var("DATABASE_URL").context("Missing `DATABASE_URL` environment variable")?;
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .max_size(max_connections())
        .max_lifetime(Some(Duration::from_secs(300)))
        .idle_timeout(Some(Duration::from_secs(60)))
        .build(manager)
        .context("Error creating connection pool")?;
    Ok(pool)
}
