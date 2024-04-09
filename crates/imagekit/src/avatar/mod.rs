mod avatar;
pub(crate) mod color;
mod scale;

pub use avatar::*;
pub use color::RgbColor;
pub use scale::Scale;

#[cfg(feature = "cli")]
pub(crate) mod utils;
