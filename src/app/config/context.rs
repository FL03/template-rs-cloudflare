/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::ApiSettings;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ApiContext {
    pub(crate) config: ApiSettings,
}

unsafe impl Send for ApiContext {}

unsafe impl Sync for ApiContext {}
