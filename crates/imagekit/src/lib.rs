#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();

pub mod download;
pub mod juliafatou;
pub mod utils;

pub mod avatar;
pub mod exif;
pub mod ogimage;
