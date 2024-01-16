use anyhow::Result;
use imagekit_cli::handle;

#[tokio::main]
async fn main() -> Result<()> {
    handle().await
}
