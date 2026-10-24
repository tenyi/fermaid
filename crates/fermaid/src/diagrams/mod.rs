//! The diagram types, one module each behind its own Cargo feature.

#[cfg(feature = "pie")]
pub(crate) mod common;
#[cfg(feature = "pie")]
pub(crate) mod pie;
