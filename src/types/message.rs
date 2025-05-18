/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Id, Timestamp};

#[derive(
    Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct Message<T = String> {
    id: Id,
    message: String,
    timestamp: Timestamp,
    data: Option<T>,
}

impl<T> Message<T> {
    pub fn new(message: impl ToString, data: Option<T>) -> Self {
        Self {
            id: Id::generate(),
            data,
            message: message.to_string(),
            timestamp: crate::systime(),
        }
    }
    /// returns a new instance constructed from the given message
    pub fn from_message(message: impl ToString) -> Self {
        Self::new(message, None)
    }
    // #[cfg(feature = "json")]
    pub fn into_json_axum(self) -> axum::Json<Message<T>> {
        axum::Json(self)
    }
    /// returns an immutable reference to the message data
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
    /// returns a mutable reference to the message data
    pub fn data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }
    /// returns a reference to the message identifier
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    /// returns a reference to the message
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
    /// returns a copy of the messages timestamp
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    pub fn set_data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self.on_update()
    }

    pub fn set_id(&mut self, id: impl ToString) -> &mut Self {
        self.id.set(id.to_string());
        self.on_update()
    }

    pub fn set_message(&mut self, message: impl ToString) -> &mut Self {
        self.message = message.to_string();
        self.on_update()
    }

    pub fn set_timestamp(&mut self, timestamp: Timestamp) -> &mut Self {
        self.timestamp = timestamp;
        self.on_update()
    }

    pub(crate) fn on_update(&mut self) -> &mut Self {
        self.set_timestamp(crate::systime());
        self
    }
}

impl<T> core::fmt::Debug for Message<T>
where
    T: serde::Serialize,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string_pretty(self).unwrap().as_str())
    }
}

impl<T> core::fmt::Display for Message<T>
where
    T: serde::Serialize,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

unsafe impl<T: Send> Send for Message<T> {}

unsafe impl<T: Sync> Sync for Message<T> {}
