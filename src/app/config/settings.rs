/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

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

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub const DEFAULT_DB_URI: &str = "sqlite://:memory:";
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: DatabaseConfig::DEFAULT_DB_URI.to_string(),
        }
    }
}
