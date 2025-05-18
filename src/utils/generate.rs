#[cfg(feature = "uuid")]
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(not(feature = "uuid"))]
pub fn generate_id() -> String {
    crate::ROOT_ATOMIC_IDENTIFIER
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        .to_string()
}
