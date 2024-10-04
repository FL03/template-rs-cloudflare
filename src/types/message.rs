/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Message {
    message: String,
}

impl Message {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn as_axum_json(&self) -> axum::Json<Self> {
        axum::Json(self.clone())
    }
}

impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string_pretty(self).unwrap().as_str())
    }
}

impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}
