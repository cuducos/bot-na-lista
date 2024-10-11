use anyhow::Result;
use chrono::{Duration, Utc};
use diesel::{delete, BoolExpressionMethods, ExpressionMethods, RunQueryDsl};

use crate::{db, schema::list};

pub fn clean_up() -> Result<()> {
    let mut conn = db::from_env()?;
    let one_year_ago = Utc::now() - Duration::days(366); // considering leap years
    delete(list::table)
        .filter(
            list::dsl::updated_at
                .lt(one_year_ago)
                .or(list::dsl::is_empty.eq(true)),
        )
        .execute(&mut conn)?;
    Ok(())
}
