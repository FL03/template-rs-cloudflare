#[doc(hidden)]
pub static ROOT_ATOMIC_IDENTIFIER: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[cfg(feature = "rand")]
pub fn generate_id() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    (0..64)
        .map(|_| rng.sample(rand::distr::Alphanumeric) as char)
        .collect::<String>()
}

#[cfg(not(feature = "rand"))]
pub fn generate_id() -> String {
    ROOT_ATOMIC_IDENTIFIER
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        .to_string()
}
