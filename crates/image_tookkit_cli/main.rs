use anyhow::Result;
use image_toolkit_cli::handle;

#[tokio::main]
async fn main() -> Result<()> {
    handle().await
}
