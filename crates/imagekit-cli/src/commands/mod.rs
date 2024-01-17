mod download;
mod juliafatou;
use crate::helpers::process_error_output;
use anyhow::Result;
use clap::{Parser, Subcommand};

/// Download image from url
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

    /// Generate an Julia Fatou image.
    /// example: cargo run --package imagekit_cli --bin imagekit juliafatou --blur 0.6 --scale 1 -c eleven --complex -0.4,0.6 -w 3
    Juliafatou(juliafatou::JuliafatouArgs),
}

pub async fn handle() -> Result<()> {
    let args = Args::parse();

    let result = match args.action {
        Action::Download(args) => download::download(args).await,
        Action::Juliafatou(args) => juliafatou::gen_julia_fatou(args).await,
    };

    process_error_output(result)
}
