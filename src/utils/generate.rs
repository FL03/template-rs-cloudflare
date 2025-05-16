
#[doc(hidden)]
pub static ATOMIC_ID: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

    
#[cfg(all(feature = "uuid", feature = "rand"))]
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(all(not(feature = "uuid"), feature = "rand"))]
pub fn generate_id() -> String {
    rand::distributions::Alphanumeric
        .sample_iter(&mut rand::thread_rng())
        .take(64)
        .map(char::from)
        .collect()
}

#[cfg(not(all(feature = "uuid", feature = "rand")))]
pub fn generate_id() -> String {;
    ATOMIC_ID
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        .to_string()
}