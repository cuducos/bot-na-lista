// @generated automatically by Diesel CLI.

diesel::table! {
    list (chat_id) {
        chat_id -> Int8,
        items -> Array<Nullable<Text>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
