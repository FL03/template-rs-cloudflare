/*
    Appellation: sample <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::aliases::{Timestamp, ItemId};

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(default, rename_all = "snake_case")]
// #[sqlx(default, rename_all = "snake_case")]
pub struct ItemModel {
    pub id: ItemId,
    pub title: String,
    pub description: String,
    pub created_at: Timestamp,
}
impl ItemModel {
    pub fn new() -> Self {
        let created_at = crate::systime();
        Self {
            id: uuid::Uuid::new_v4(),
            description: String::new(),
            title: String::new(),
            created_at,
        }
    }

    gsw! {
        id: ItemId,
        created_at: Timestamp,
    }

    gsw! {
        description: &String,
        title: &String,
    }
}

pub struct ItemBuilder {
    id: Option<ItemId>,
    description: Option<String>,
    title: Option<String>,
}

impl ItemBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            description: None,
            title: None,
        }
    }

    pub fn description(self, description: impl ToString) -> Self {
        Self {
            description: Some(description.to_string()),
            ..self
        }
    }

    pub fn id(self, id: ItemId) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }

    pub fn title(self, title: impl ToString) -> Self {
        Self {
            title: Some(title.to_string()),
            ..self
        }
    }

    pub fn build(self) -> ItemModel {
        ItemModel {
            id: self.id.unwrap_or_else(uuid::Uuid::new_v4),
            description: self.description.unwrap_or_default(),
            title: self.title.unwrap_or_default(),
            created_at: crate::systime(),
        }
    }
}
