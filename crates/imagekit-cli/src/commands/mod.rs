mod download_image;
mod juliafatou;
use crate::{
    helpers::process_error_output,
    parsers::{parse_image_formats, parse_url},
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use image::ImageFormat;
use url::Url;

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
    Run(RunArgs),

    /// Generate an Julia Fatou image.
    /// example: cargo run --package imagekit_cli --bin imagekit juliafatou --blur 0.6 --scale 1 -c eleven --complex -0.4,0.6 -w 3
    Juliafatou(juliafatou::JuliafatouArgs),
}

#[derive(Parser, Debug, Clone)]
struct RunArgs {
    /// The url of the image
    /// e.g. -u https://t7.baidu.com/it/u=1595072465,3644073269&fm=193&f=GIF
    #[clap(short, long, value_parser = parse_url)]
    url: Option<Url>,

    /// The image format to download, support multiple formats.
    /// Supported formats: png (png)、jpeg (jpg)、webp (webp)、gif (gif)、avif (avif)、bmp (bmp)、tiff (tiff)、ico (ico)
    #[clap(short, long, value_parser = parse_image_formats, value_delimiter = ' ', num_args = 1..)]
    formats: Option<Vec<ImageFormat>>,

    /// The filename of the output image
    #[clap(short = 'F', long, default_value_t = String::from("image"), value_name = "FILE NAME")]
    filename: String,
}

pub async fn handle() -> Result<()> {
    let args = Args::parse();

    let result = match args.action {
        Action::Run(args) => download_image::run(args).await,
        Action::Juliafatou(args) => juliafatou::gen_julia_fatou(args).await,
    };

    process_error_output(result)
}
