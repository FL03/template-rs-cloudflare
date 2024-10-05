/*
    Appellation: sample <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{DateT, ItemId, Timezone};

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
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: ItemId,
    pub title: String,
    pub description: String,
    #[serde(default = "chrono::Local::now")]
    pub created_at: DateT,
}

impl ItemModel {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            description: String::new(),
            title: String::new(),
            created_at: Timezone::now(),
        }
    }

    setwith! {
        description: String,
        id: ItemId,
        title: String,
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn id(&self) -> ItemId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn created_at(&self) -> DateT {
        self.created_at
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
            created_at: Timezone::now(),
        }
    }
}
