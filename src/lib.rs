use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub mod chat;

pub mod db;
pub mod models;
pub mod schema;
pub mod telegram;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
