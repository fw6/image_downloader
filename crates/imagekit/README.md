# Image toolkit

图片相关操作工具集

## Usage

```toml
[dependencies]
imagekit = { "*", features = ["secrets"] }
```

## Minimum Supported Rust Version

Rust 1.57 or higher.

## Example

```rust
use imagekit::{download_image_by_url, images_to_video};

async fn test() -> Result<()> {
    download_image_by_url().await?;
    Ok(())
}
```
