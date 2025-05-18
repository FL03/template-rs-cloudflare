/*
    Appellation: statics <module>
    Contrib: @FL03
*/

use core::sync::atomic::AtomicUsize;

/// a statically declared [`AtomicUsize`] instance that is used to generate unique IDs
pub static ROOT_ATOMIC_IDENTIFIER: AtomicUsize = AtomicUsize::new(0);