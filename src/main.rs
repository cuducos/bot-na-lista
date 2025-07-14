use std::time::Duration;

use anyhow::{Context, Result};
use bot_na_lista::{db, maintenance, telegram, MIGRATIONS};
use diesel_migrations::{HarnessWithOutput, MigrationHarness};
use tokio::{spawn, time::sleep};

const ONE_DAY: Duration = Duration::from_secs(60 * 60 * 24);

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting Bot na Lista");
    let pool = db::pool_from_env().context("Could not get a database connection pool")?;
    let mut conn = pool.get().context("Could not get a database connection")?;
    let mut harness = HarnessWithOutput::write_to_stdout(&mut conn);
    harness
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Failed to run pending migrations: {}", e))?;
    spawn(async {
        loop {
            if let Err(e) = maintenance::clean_up() {
                log::error!("Error cleaning up: {e:?}");
            }
            sleep(ONE_DAY).await;
        }
    });
    telegram::run(pool).await
}
