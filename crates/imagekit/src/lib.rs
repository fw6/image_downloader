#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();

pub mod download;
mod juliafatou;
pub mod utils;

pub use juliafatou::JuliafatouBuilder;
