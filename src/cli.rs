use anyhow::{anyhow, Result};

pub fn parse_url(s: &str) -> Result<url::Url> {
    url::Url::parse(s).map_err(|e| anyhow!("Invalid url: {}", e))
}

pub fn parse_image_formats(s: &str) -> Result<image::ImageFormat> {
    match s {
        "png" => Ok(image::ImageFormat::Png),
        "jpeg" => Ok(image::ImageFormat::Jpeg),
        "webp" => Ok(image::ImageFormat::WebP),
        "gif" => Ok(image::ImageFormat::Gif),
        "avif" => Ok(image::ImageFormat::Avif),
        "bmp" => Ok(image::ImageFormat::Bmp),
        "tiff" => Ok(image::ImageFormat::Tiff),
        "ico" => Ok(image::ImageFormat::Ico),
        _ => Err(anyhow!("Invalid image format")),
    }
}

pub fn parse_path(s: &str) -> Result<std::path::PathBuf> {
    Ok(std::path::PathBuf::from(s))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_url_should_work() {
        let url = parse_url("https://www.baidu.com").unwrap();
        assert_eq!(url.host_str(), Some("www.baidu.com"));
    }

    #[test]
    fn parse_image_formats_should_work() {
        let format = parse_image_formats("png").unwrap();
        assert_eq!(format, image::ImageFormat::Png);
    }
}
