use bot_na_lista::{chat::Chat, db};
use common::run;

mod common;

#[test]
fn test_add_item() {
    run(|| {
        let pool = db::pool_from_env().expect("Could not connect to PostgreSQL");
        let mut chat = Chat::new(42, &pool).expect("Could not start chat");
        let list = chat
            .process_input("Foo")
            .expect("Could not add Foo to the list");
        assert!(!list.is_empty);
        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items.first(), Some(&Some("Foo".to_string())));
    });
}

#[test]
fn test_add_items() {
    run(|| {
        let pool = db::pool_from_env().expect("Could not connect to PostgreSQL");
        let mut chat = Chat::new(42, &pool).expect("Could not start chat");
        chat.process_input("Foo")
            .expect("Could not add Foo to the list");
        let list = chat
            .process_input("Bar")
            .expect("Could not add Bar to the list");
        assert!(!list.is_empty);
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items.first(), Some(&Some("Foo".to_string())));
        assert_eq!(list.items.get(1), Some(&Some("Bar".to_string())));
    });
}

#[test]
fn test_add_duplicated_item() {
    run(|| {
        let pool = db::pool_from_env().expect("Could not connect to PostgreSQL");
        let mut chat = Chat::new(42, &pool).expect("Could not start chat");
        chat.process_input("Foo")
            .expect("Could not add Foo to the list");
        chat.process_input("Bar")
            .expect("Could not add Bar to the list");
        let list = chat
            .process_input("Foo")
            .expect("Could not add Foo to the list for the second time");
        assert!(!list.is_empty);
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items.first(), Some(&Some("Foo".to_string())));
        assert_eq!(list.items.get(1), Some(&Some("Bar".to_string())));
    });
}

#[test]
fn test_remove_item() {
    run(|| {
        let pool = db::pool_from_env().expect("Could not connect to PostgreSQL");
        let mut chat = Chat::new(42, &pool).expect("Could not start chat");
        chat.process_input("Foo")
            .expect("Could not add Foo to the list");
        chat.process_input("Bar")
            .expect("Could not add Bar to the list");
        let list = chat
            .process_input("1")
            .expect("Could not remove the first item of the list");
        assert!(!list.is_empty);
        assert_eq!(list.items.len(), 1);
        assert!(!list.items.contains(&Some("Foo".to_string())));
        assert_eq!(list.items.first(), Some(&Some("Bar".to_string())));
    });
}

#[test]
fn test_remove_invalid_index() {
    run(|| {
        let pool = db::pool_from_env().expect("Could not connect to PostgreSQL");
        let mut chat = Chat::new(42, &pool).expect("Could not start chat");
        chat.process_input("Foo")
            .expect("Could not add Foo to the list");
        let list = chat
            .process_input("2")
            .expect("Could not remove the item from the list");
        assert!(!list.is_empty);
        assert_eq!(list.items.len(), 1);
        assert!(list.items.contains(&Some("Foo".to_string())));
    });
}

#[test]
fn test_remove_last_item_sets_is_empty() {
    run(|| {
        let pool = db::pool_from_env().expect("Could not connect to PostgreSQL");
        let mut chat = Chat::new(42, &pool).expect("Could not start chat");
        chat.process_input("Foo")
            .expect("Could not add Foo to the list");
        let list = chat
            .process_input("1")
            .expect("Could not remove the item from the list");
        assert!(list.is_empty);
        assert_eq!(list.items.len(), 0);
    });
}
