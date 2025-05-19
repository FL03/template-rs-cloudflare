/*
  Appellation: time <module>
  Contrib: @FL03
*/

/// [systime] is a method that returns the current time, in milliseconds, using the
/// [`SystemTime`](std::time::SystemTime) API.
#[cfg(not(any(feature = "wasm", target_arch = "wasm32")))]
#[inline]
pub fn systime() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .expect("time went backwards")
}

#[inline]
/// the [`timestamp`] method returns a numerical reprsentation of the current time, in
/// milliseconds.
pub fn timestamp() -> crate::Timestamp {
    chrono::Utc::now().timestamp_millis()
}
