use crate::{
    models::{List, NewList},
    schema::list,
};
use anyhow::{Context, Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    update, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
};

pub struct Chat {
    pub chat_id: i64,
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl Chat {
    pub fn new(chat_id: i64, conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self { chat_id, conn }
    }

    fn create_list(&mut self) -> Result<List> {
        diesel::insert_into(list::dsl::list)
            .values(&NewList {
                chat_id: self.chat_id,
            })
            .get_result::<List>(&mut self.conn)
            .context(format!("[List#{}] Error creating list", self.chat_id))
    }

    fn add_item(&mut self, item: &str) -> Result<List> {
        let new_item = Some(item.trim().to_string());
        let mut list = self.list().context(format!(
            "[List#{}] Error looking for list to add item",
            self.chat_id
        ))?;
        if !list.items.contains(&new_item) {
            list.items.push(new_item);
            update(list::table)
                .filter(list::dsl::chat_id.eq(self.chat_id))
                .set(&list)
                .execute(&mut self.conn)
                .context(format!("[List#{}] Error saving new item", self.chat_id))?;
        }
        Ok(list)
    }

    fn remove_item(&mut self, item: usize) -> Result<List> {
        let mut list = self.list().context(format!(
            "[List#{}] Error looking for list to remove item",
            self.chat_id
        ))?;
        if item < list.items.len() {
            list.items.remove(item);
            update(list::table)
                .filter(list::dsl::chat_id.eq(self.chat_id))
                .set(&list)
                .execute(&mut self.conn)
                .context(format!("[List#{}] Error removing item", self.chat_id))?;
        }
        Ok(list)
    }

    pub fn list(&mut self) -> Result<List> {
        let existing = list::dsl::list
            .filter(list::dsl::chat_id.eq(self.chat_id))
            .first::<List>(&mut self.conn)
            .optional()
            .context(format!("[List#{}] Error looking for list", self.chat_id))?;

        if let Some(list) = existing {
            log::info!("Using existing List#{}", self.chat_id);
            return Ok(list);
        }

        log::info!("Creting new List#{}", self.chat_id);
        self.create_list()
            .context(format!("[List#{}] Error creating new list", self.chat_id))
    }

    pub fn process_input(&mut self, item: &str) -> Result<List> {
        if let Ok(index) = item.parse::<usize>() {
            self.remove_item(index - 1)
        } else {
            self.add_item(item)
        }
    }
}