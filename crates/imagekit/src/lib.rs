#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();

pub mod download;
mod juliafatou;
pub mod utils;

pub use juliafatou::{ColorStyle, Juliafatou, JuliafatouBuilder};
pub mod ogimage;
