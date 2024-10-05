/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

display!(json(DatabaseSettings, Settings));

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct DatabaseSettings {
    pub url: String,
}

pub mod db {
    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
    )]
    pub struct DbUriBuilder {
        pub connection: String,
        pub host: String,
        pub port: u16,
        pub user: String,
        pub password: String,
        pub database: String,
    }
}
