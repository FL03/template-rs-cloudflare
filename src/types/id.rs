/*
    Appellation: id <module>
    Contrib: @FL03
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
)]
#[repr(transparent)]
#[serde(default, transparent)]
pub struct Id<T = String>(pub T);

impl<T> Id<T> {
    pub fn new(id: T) -> Self {
        Self(id)
    }
    /// returns a new [`Id`] instance with the given value
    pub fn from_value(value: T) -> Self {
        Self(value)
    }
    /// consumes the current instance to return the inner value
    pub fn into_inner(self) -> T {
        self.0
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// apply a function to the inner value and return a new [`Id`] instance
    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Id<U> {
        Id(f(self.get()))
    }
    /// apply a mutable function to the inner value and return a mutable reference to the
    /// current instance
    pub fn map_mut<F: FnMut(&mut T)>(&mut self, mut f: F) -> &mut Self {
        f(self.get_mut());
        self
    }
    /// replace and return the previous state with the given value
    pub fn replace(&mut self, value: T) -> T {
        core::mem::replace(&mut self.0, value)
    }
    /// updates the inner value with the given value
    pub fn set(&mut self, value: T) -> &mut Self {
        self.0 = value;
        self
    }
    /// swaps the inner value with another [`Id`] instance
    pub fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.0, &mut other.0);
    }
    /// returns a new instance using the string representation of the inner value
    pub fn as_id_string(&self) -> Id<String>
    where
        T: ToString,
    {
        self.map(|id| id.to_string())
    }
}

impl<T> AsRef<T> for Id<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Id<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::borrow::Borrow<T> for Id<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for Id<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Id<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Id<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Id<String> {
    /// generate a new unique identifier;
    ///
    /// if the `uuid` feature is enabled, a [`uuid::Uuid`] is generated and converted to a
    /// string; optherwise, an atomic counter is used to generate a unique identifier
    pub fn generate() -> Self {
        #[cfg(feature = "uuid")]
        return Id::v4().as_string_id();
        #[cfg(not(feature = "uuid"))]
        return Self::atomic().as_id_string();
    }
}

impl Id<usize> {
    pub fn atomic() -> Self {
        Self(crate::ROOT_ATOMIC_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
}

#[cfg(feature = "uuid")]
impl Id<uuid::Uuid> {
    pub fn v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn as_string_id(&self) -> Id<String> {
        self.map(|id| id.to_string())
    }
}
