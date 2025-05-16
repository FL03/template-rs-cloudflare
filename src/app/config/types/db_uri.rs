/*
    Appellation: connection <module>
    Contrib: @FL03
*/
/// An enumeration of supported database drivers and the formats that we expect them to be in.
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
    strum::EnumString,
    strum::EnumIs,
    strum::EnumIter,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DatabaseDrivers {
    #[serde(alias = "postgresql", alias = "postgres")]
    #[strum(to_string = "postgresql")]
    Postgresql,
    #[serde(alias = "mysql")]
    Mysql,
    #[default]
    #[serde(alias = "sqlite", alias = "sqlite3")]
    #[strum(to_string = "sqlite")]
    Sqlite,
}

/// Extends the standard [`DatabaseUriSchema`] connection schema with additional options for
/// alternative resource types such as redis, object-storage, etc.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "snake_case")]
pub enum StorageProvider {
    Sql(DatabaseUriSchema),
}

/// A standard database connection url schema
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct DatabaseUriSchema {
    pub(crate) prefix: DatabaseDrivers,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) database: String,
}

impl DatabaseUriSchema {
    pub fn from_parts(
        prefix: DatabaseDrivers,
        host: impl ToString,
        port: u16,
        user: impl ToString,
        password: impl ToString,
        database: impl ToString,
    ) -> Self {
        Self {
            prefix,
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            database: database.to_string(),
        }
    }
    /// returns a new instance pre-configured with the postgresql prefix.
    pub fn postgresql(
        host: impl ToString,
        port: u16,
        user: impl ToString,
        password: impl ToString,
        database: impl ToString,
    ) -> Self {
        Self::from_parts(
            DatabaseDrivers::Postgresql,
            host,
            port,
            user,
            password,
            database,
        )
    }
    /// returns a string representation of the database URL.
    pub fn to_string(&self) -> String {
        format!(
            "{prefix}://{user}:{password}@{host}:{port}/{database}",
            prefix = self.prefix,
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            database = self.database
        )
    }
    /// returns a reference to the driver
    pub const fn prefix(&self) -> DatabaseDrivers {
        self.prefix
    }
    /// returns a mutable reference to the driver
    #[inline]
    pub fn prefix_mut(&mut self) -> &mut DatabaseDrivers {
        &mut self.prefix
    }
    /// returns a reference to the host
    #[inline]
    pub fn host(&self) -> &str {
        &self.host
    }
    /// returns a mutable reference to the host
    #[inline]
    pub fn host_mut(&mut self) -> &mut String {
        &mut self.host
    }
    /// returns a copy of the configured port
    pub const fn port(&self) -> u16 {
        self.port
    }
    /// returns a mutable reference to the port
    #[inline]
    pub fn port_mut(&mut self) -> &mut u16 {
        &mut self.port
    }
    /// returns a reference to the username
    #[inline]
    pub fn user(&self) -> &str {
        &self.user
    }
    /// returns a mutable reference to the username
    #[inline]
    pub fn user_mut(&mut self) -> &mut String {
        &mut self.user
    }
    /// returns a reference to the password
    #[inline]
    pub fn password(&self) -> &str {
        &self.password
    }
    /// returns a mutable reference to the password
    #[inline]
    pub fn password_mut(&mut self) -> &mut String {
        &mut self.password
    }
    /// returns a reference to the database name
    #[inline]
    pub fn database(&self) -> &str {
        &self.database
    }
    /// returns a mutable reference to the database name
    #[inline]
    pub fn database_mut(&mut self) -> &mut String {
        &mut self.database
    }
    /// update the configured database and return a mutable reference to the current instance
    #[inline]
    pub fn set_database<U: ToString>(&mut self, value: U) -> &mut Self {
        *self.database_mut() = value.to_string();
        self
    }
    /// update the configured hostname and return a mutable reference to the current instance
    #[inline]
    pub fn set_host(&mut self, host: impl ToString) -> &mut Self {
        self.host = host.to_string();
        self
    }
    /// update the configured password and return a mutable reference to the current instance
    #[inline]
    pub fn set_password<U: ToString>(&mut self, value: U) -> &mut Self {
        *self.password_mut() = value.to_string();
        self
    }
    /// update the configured port and return a mutable reference to the current instance
    #[inline]
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    /// update the configured prefix and return a mutable reference to the current instance
    #[inline]
    pub fn set_prefix(&mut self, value: DatabaseDrivers) -> &mut Self {
        *self.prefix_mut() = value;
        self
    }
    /// update the configured username and return a mutable reference to the current instance
    #[inline]
    pub fn set_user<U: ToString>(&mut self, value: U) -> &mut Self {
        *self.user_mut() = value.to_string();
        self
    }
    /// consumes the current instance to create another with the given database name
    pub fn with_database<U: ToString>(self, value: U) -> Self {
        Self {
            database: value.to_string(),
            ..self
        }
    }
    /// consumes the current instance to create another with the given host
    pub fn with_host<U: ToString>(self, value: U) -> Self {
        Self {
            host: value.to_string(),
            ..self
        }
    }
    /// consumes the current instance to create another with the given password
    pub fn with_password<U: ToString>(self, value: U) -> Self {
        Self {
            password: value.to_string(),
            ..self
        }
    }
    /// consumes the current instance to create another with the given port
    pub fn with_port(self, value: u16) -> Self {
        Self {
            port: value,
            ..self
        }
    }
    /// consumes the current instance to create another with the given prefix
    pub fn with_prefix(self, value: DatabaseDrivers) -> Self {
        Self {
            prefix: value,
            ..self
        }
    }
    /// consumes the current instance to create another with the given username
    pub fn with_user<U: ToString>(self, value: U) -> Self {
        Self {
            user: value.to_string(),
            ..self
        }
    }
}

impl Default for DatabaseUriSchema {
    fn default() -> Self {
        Self {
            prefix: DatabaseDrivers::Postgresql,
            host: "localhost".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            password: "password".to_string(),
            database: "database".to_string(),
        }
    }
}

impl core::fmt::Display for DatabaseUriSchema {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.to_string())
    }
}

// impl core::str::FromStr for DatabaseUriSchema {
//     type Err = anyhow::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let url = url::Url::parse(s)?;
//         let host = url
//             .host_str()
//             .ok_or(anyhow::anyhow!("No hostname parsed"))?;
//         let port = url.port().unwrap_or(5432);
//         let user = url.username();
//         let password = url.password().unwrap_or("");
//         let database = url.path().trim_start_matches('/');

//         Ok(Self {
//             prefix: DatabaseDrivers::from_str(url.scheme())?,
//             host: host.to_string(),
//             port,
//             user: user.to_string(),
//             password: password.to_string(),
//             database: database.to_string(),
//         })
//     }
// }
