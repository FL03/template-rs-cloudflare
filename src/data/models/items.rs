/*
    Appellation: sample <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::types::aliases::{ItemId, Timestamp};

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
        let id = crate::generate_id();
        let created_at = crate::systime();
        Self {
            id,
            description: String::new(),
            title: String::new(),
            created_at,
        }
    }

    gsw! {
        created_at: Timestamp,
    }

    gsw! {
        description: &String,
        id: &ItemId,
        title: &String,
    }
}
