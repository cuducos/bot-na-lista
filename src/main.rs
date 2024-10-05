use anyhow::{Context, Result};
use bot_na_lista::{db, telegram, MIGRATIONS};
use diesel_migrations::{HarnessWithOutput, MigrationHarness};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting Bot na Lista");
    let pool = db::from_env().context("Could not get a database connection pool")?;
    let mut conn = pool.get().context("Could not get a database connection")?;
    let mut harness = HarnessWithOutput::write_to_stdout(&mut conn);
    harness
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Failed to run pending migrations: {}", e))?;
    telegram::run(pool).await
}
