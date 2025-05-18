/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::services::*;

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
#[serde(default)]
pub struct ApiSettings {
    pub database: DatabaseConfig,
}

impl ApiSettings {
    pub fn new() -> Self {
        Self {
            database: DatabaseConfig::default(),
        }
    }

    gsw! {
        database: &DatabaseConfig,
    }
}
