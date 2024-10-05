/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Message<T = String> {
    id: String,
    message: String,
    timestamp: i64,
    data: Option<T>,
}

impl<T> Message<T> {
    pub fn new(message: impl ToString, data: Option<T>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            data,
            message: message.to_string(),
            timestamp: chrono::Local::now().timestamp(),
        }
    }

    pub fn from_message(message: impl ToString) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            data: None,
            message: message.to_string(),
            timestamp: chrono::Local::now().timestamp(),
        }
    }

    pub fn into_json_axum(self) -> axum::Json<Message<T>> {
        axum::Json(self)
    }

    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
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
