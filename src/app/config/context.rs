/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::ApiSettings;

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
pub struct ApiContext {
    pub(crate) config: ApiSettings,
}

impl ApiContext {
    pub fn new() -> Self {
        Self {
            config: ApiSettings::default(),
        }
    }
    /// returns an immutable reference to the [`config`](ApiSettings)
    pub const fn config(&self) -> &ApiSettings {
        &self.config
    }
    /// returns a mutable reference to the config
    pub const fn config_mut(&mut self) -> &mut ApiSettings {
        &mut self.config
    }
    /// update the current config then return a mutable reference to the context
    pub fn set_config(&mut self, config: ApiSettings) -> &mut Self {
        self.config = config;
        self
    }
    /// consumes the current context to create another with the given configuration
    pub fn with_config(self, config: ApiSettings) -> Self {
        Self { config, ..self }
    }
}

impl core::ops::Deref for ApiContext {
    type Target = ApiSettings;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl core::ops::DerefMut for ApiContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

unsafe impl Send for ApiContext {}

unsafe impl Sync for ApiContext {}
