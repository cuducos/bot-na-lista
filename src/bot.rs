use crate::{
    models::{List, NewList},
    schema::list,
};
use anyhow::{Context, Result};
use diesel::{update, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

pub struct Bot {
    pub chat_id: i64,
}

impl Bot {
    pub fn new(chat_id: i64) -> Self {
        Self { chat_id }
    }

    fn create_list(&self, conn: &mut PgConnection) -> Result<List> {
        diesel::insert_into(list::dsl::list)
            .values(&NewList {
                chat_id: self.chat_id,
            })
            .get_result::<List>(conn)
            .context(format!("[List#{}] Error creating list", self.chat_id))
    }

    fn add_item(&self, conn: &mut PgConnection, item: &str) -> Result<List> {
        let new_item = Some(item.trim().to_string());
        let mut list = self.list(conn).context(format!(
            "[List#{}] Error looking for list to add item",
            self.chat_id
        ))?;
        if !list.items.contains(&new_item) {
            list.items.push(new_item);
            update(list::table)
                .filter(list::dsl::chat_id.eq(self.chat_id))
                .set(&list)
                .execute(conn)
                .context(format!("[List#{}] Error saving new item", self.chat_id))?;
        }
        Ok(list)
    }

    fn remove_item(&self, conn: &mut PgConnection, item: usize) -> Result<List> {
        let mut list = self.list(conn).context(format!(
            "[List#{}] Error looking for list to remove item",
            self.chat_id
        ))?;
        if item < list.items.len() {
            list.items.remove(item);
            update(list::table)
                .filter(list::dsl::chat_id.eq(self.chat_id))
                .set(&list)
                .execute(conn)
                .context(format!("[List#{}] Error removing item", self.chat_id))?;
        }
        Ok(list)
    }

    pub fn list(&self, conn: &mut PgConnection) -> Result<List> {
        let existing = list::dsl::list
            .filter(list::dsl::chat_id.eq(self.chat_id))
            .first::<List>(conn)
            .optional()
            .context(format!("[List#{}] Error looking for list", self.chat_id))?;

        if let Some(list) = existing {
            log::info!("Using existing List#{}", self.chat_id);
            return Ok(list);
        }

        log::info!("Creting new List#{}", self.chat_id);
        self.create_list(conn)
            .context(format!("[List#{}] Error creating new list", self.chat_id))
    }

    pub fn process_input(&self, conn: &mut PgConnection, item: &str) -> Result<List> {
        if let Ok(index) = item.parse::<usize>() {
            self.remove_item(conn, index - 1)
        } else {
            self.add_item(conn, item)
        }
    }
}

