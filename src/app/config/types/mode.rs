/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    #[default]
    #[serde(alias = "debug", alias = "dev")]
    Development,
    #[serde(alias = "stag")]
    Staging,
    #[serde(alias = "prod")]
    Production,
}

impl Mode {
    pub fn development() -> Self {
        Self::Development
    }

    pub fn production() -> Self {
        Self::Production
    }

    pub fn staging() -> Self {
        Self::Staging
    }
}

impl From<Mode> for config::Value {
    fn from(mode: Mode) -> Self {
        mode.to_string().into()
    }
}
