use bot_na_lista::{chat::Chat, db, maintenance::clean_up, models::List, schema::list};
use chrono::{DateTime, Duration, Utc};
use common::run;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    update, ExpressionMethods, PgConnection, RunQueryDsl,
};

mod common;

fn list_with_date_and_item(
    pool: Pool<ConnectionManager<PgConnection>>,
    chat_id: i64,
    date: Option<DateTime<Utc>>,
    item: Option<String>,
) -> List {
    let mut chat = Chat::new(chat_id, &pool).expect("Could not start chat");
    let list = chat
        .list()
        .unwrap_or_else(|e| panic!("[List#{chat_id}] Error creating list: {e:?}"));
    if let Some(it) = item {
        Chat::new(list.chat_id, &pool)
            .expect("Could not start chat")
            .process_input(it.as_str())
            .expect("Could not add item to list");
    }
    if let Some(dt) = date {
        let mut conn = pool.get().expect("Could not get database connection");
        let affected = update(list::table)
            .filter(list::chat_id.eq(chat_id))
            .set(list::updated_at.eq(dt))
            .execute(&mut conn)
            .expect("Failed to update list updated_at");
        assert_eq!(1, affected);
    }
    list
}

#[test]
fn test_list_with_no_activity_are_deleted_after_one_year() {
    run(|| {
        let pool = db::pool_from_env().expect("Could not connect to PostgreSQL");
        let one_year_ago = Utc::now() - Duration::days(367); // beyond leap-year
        let one_month_ago = Utc::now() - Duration::days(30);
        list_with_date_and_item(pool.clone(), 0, Some(one_year_ago), Some("Foo".to_string()));
        let new_chat = list_with_date_and_item(
            pool.clone(),
            42,
            Some(one_month_ago),
            Some("Bar".to_string()),
        );
        let mut conn = pool.get().expect("Could not get database connection");
        clean_up().expect("Error cleaning up");
        let lists = list::dsl::list
            .load::<List>(&mut conn)
            .expect("Failed to get lists");
        assert_eq!(lists.len(), 1, "expected 1 list, got\n{lists:?}");
        assert_eq!(lists[0].chat_id, new_chat.chat_id);
    });
}

#[test]
fn test_list_with_no_items_are_deleted() {
    run(|| {
        let pool = db::pool_from_env().expect("Could not connect to PostgreSQL");
        list_with_date_and_item(pool.clone(), 0, None, None);
        let list_with_item =
            list_with_date_and_item(pool.clone(), 42, None, Some("Foo".to_string()));
        clean_up().expect("Error cleaning up");
        let mut conn = pool.get().expect("Could not get database connection");
        let lists = list::dsl::list
            .load::<List>(&mut conn)
            .expect("Failed to get lists");
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].chat_id, list_with_item.chat_id);
    });
}
