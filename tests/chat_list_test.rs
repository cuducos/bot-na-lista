use bot_na_lista::{
    chat::Chat,
    db,
    models::{List, NewList},
    schema::list,
};
use common::run;
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, QueryDsl, RunQueryDsl,
};

mod common;

fn assert_list_table_count(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    expected: i64,
) {
    let count = list::dsl::list
        .count()
        .get_result::<i64>(conn)
        .expect("Could not get list count");
    assert_eq!(
        count, expected,
        "expected list table to have {} rows, got {}",
        expected, count
    );
}

#[test]
fn test_new_chat_gets_a_new_list() {
    run(|| {
        let pool = db::from_env().expect("Could not connect to PostgreSQL");
        let mut chat = Chat::new(42, pool.get().expect("Could not get database connection"));
        let mut conn = pool.get().expect("Could not get database connection");
        assert_list_table_count(&mut conn, 0);
        let list = chat.list().expect("Error getting a list");
        assert_eq!(42, list.chat_id);
        assert_list_table_count(&mut conn, 1);
    });
}

#[test]
fn test_known_chat_gets_existing_list() {
    run(|| {
        let pool = db::from_env().expect("Could not connect to PostgreSQL");
        let mut conn = pool.get().expect("Could not get database connection");
        assert_list_table_count(&mut conn, 0);
        insert_into(list::table)
            .values(&NewList { chat_id: 42 })
            .get_result::<List>(&mut conn)
            .expect("Could not create existing list");
        assert_list_table_count(&mut conn, 1);
        let mut chat = Chat::new(42, conn);
        let list = chat.list().expect("Error getting a list");
        assert_eq!(42, list.chat_id);
    });
}
