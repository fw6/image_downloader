mod download;
mod juliafatou;
use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::helpers::process_error_output;

/// The imagekit cli includes a set of commands for image processing.
/// - download: Download image from url
/// - juliafatou: Generate an Julia Fatou image set.
/// - ...
#[derive(Parser, Debug, Clone)]
#[clap(version = "0.1.0", author = "feng.w <feng.w@trip.com>")]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug, Clone)]
#[non_exhaustive]
enum Action {
    /// Download image from url
    Download(download::DownloadArgs),

    /// Generate an Julia Fatou image set.
    /// example: cargo run --package imagekit_cli --bin imagekit juliafatou
    /// --blur 0.6 --scale 1 -c eleven --complex -0.4,0.6 -w 3
    Juliafatou(juliafatou::JuliafatouArgs),
}

pub async fn handle() -> Result<()> {
    let args = Args::parse();

    let result = match args.action {
        Action::Download(args) => download::download(args).await,
        Action::Juliafatou(args) => juliafatou::gen_juliafatou(args).await,
    };

    process_error_output(result)
}
