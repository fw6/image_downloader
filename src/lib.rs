pub mod cli;
pub mod commands;
mod utils;
pub use utils::{download_image, process_error_output};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Env {
    Dev,
    Test,
    Prod,
}
