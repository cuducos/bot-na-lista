use chrono::{DateTime, Utc};
use diesel::{prelude::AsChangeset, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::list)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct List {
    pub chat_id: i64,
    pub items: Vec<Option<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_empty: bool,
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.items.is_empty() {
            write!(f, "The list is empty ¯\\_(ツ)_/¯")?;
        } else {
            for (index, item) in self.items.iter().enumerate() {
                if let Some(txt) = item {
                    writeln!(f, "{}. {}", index + 1, txt)?;
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("chat_id", &self.chat_id)
            .field("is_empty", &self.is_empty)
            .field("items", &self.items)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::list)]
pub struct NewList {
    pub chat_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_display() {
        let list = List {
            chat_id: 42,
            is_empty: false,
            items: vec![Some("Foo".to_string()), Some("Bar".to_string()), None],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let expected_output = "1. Foo\n2. Bar\n";
        assert_eq!(format!("{list}"), expected_output);
    }

    #[test]
    fn test_empty_list_display() {
        let list = List {
            chat_id: 1,
            is_empty: true,
            items: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let expected_output = "The list is empty ¯\\_(ツ)_/¯";
        assert_eq!(format!("{list}"), expected_output);
    }
}
