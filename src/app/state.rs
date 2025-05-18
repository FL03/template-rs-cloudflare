/*
    Appellation: state <module>
    Contrib: @FL03
*/

#[derive(
    Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default, rename_all = "snake_case")]
pub struct AppState {
    pub(crate) user: Option<UserState>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default, rename_all = "snake_case")]
pub struct UserState {
    pub(crate) username: String,
}

impl UserState {
    pub fn from_username(username: String) -> Self {
        Self {
            username,
        }
    }
    /// returns a reference to the current username
    pub const fn username(&self) -> &String {
        &self.username
    }
    /// returns a mutable reference to the current username
    pub fn username_mut(&mut self) -> &mut String {
        &mut self.username
    }
    /// updates the current username and return a mutable reference to the state
    pub fn set_username<T: ToString>(&mut self, value: T) -> &mut Self {
        self.username = value.to_string();
        self
    }
    /// consumes the current state to create another with the given username
    pub fn with_username<T: ToString>(self, value: T) -> Self {
        Self {
            username: value.to_string(),
            ..self
        }
    }
}

impl Default for UserState {
    fn default() -> Self {
        let username = String::new();
        Self::from_username(username)
    }
}


impl AppState {
    pub fn new() -> Self {
        Self {
            user: None,
        }
    }

    pub fn user(&self) -> Option<&UserState> {
        self.user.as_ref()
    }

    pub fn user_mut(&mut self) -> Option<&mut UserState> {
        self.user.as_mut()
    }

    pub fn set_user(&mut self, user: UserState) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn with_user(&self, user: UserState) -> Self {
        Self {
            user: Some(user),
            ..self.clone()
        }
    }
}

