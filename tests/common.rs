use diesel_migrations::{HarnessWithOutput, MigrationHarness};
use std::panic::{catch_unwind, UnwindSafe};

use bot_na_lista::{db, MIGRATIONS};

pub fn run<T>(test: T)
where
    T: FnOnce() + UnwindSafe,
{
    let pool = db::from_env().expect("Could not connect to PostgreSQL");
    let mut conn = pool.get().expect("Could not get a PostgreSQL connection");
    let mut harness = HarnessWithOutput::write_to_stdout(&mut conn);
    harness
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");
    let result = catch_unwind(test);
    harness
        .revert_all_migrations(MIGRATIONS)
        .expect("Error cleaning up the database");
    assert!(result.is_ok())
}
