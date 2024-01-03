use anyhow::Result;
use image_downloader::commands::handle;

#[tokio::main]
async fn main() -> Result<()> {
    handle().await
}
