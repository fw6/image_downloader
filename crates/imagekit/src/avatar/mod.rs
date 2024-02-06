mod avatar;
pub(crate) mod color;
pub(crate) mod scale;
pub use avatar::*;

#[cfg(feature = "cli")]
pub(crate) mod utils;
