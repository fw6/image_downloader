use anyhow::{anyhow, Result};
use image::ImageFormat;
use mime::Mime;
use reqwest::header::CONTENT_TYPE;

pub fn get_content_type(headers: &reqwest::header::HeaderMap) -> Result<mime::Mime> {
    headers
        .get(CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .map(|mime| mime.parse().unwrap_or(mime::APPLICATION_OCTET_STREAM))
        .ok_or_else(|| anyhow!("Failed to get content type"))
}

pub fn mime_to_image_format(mime: Mime) -> Option<ImageFormat> {
    if mime.type_() != mime::IMAGE {
        return None;
    }

    let subtype = mime.subtype().as_str();

    match subtype {
        "png" => Some(ImageFormat::Png),
        "jpeg" => Some(ImageFormat::Jpeg),
        "webp" => Some(ImageFormat::Bmp),
        "gif" => Some(ImageFormat::Gif),
        "avif" => Some(ImageFormat::Avif),
        "bmp" => Some(ImageFormat::Bmp),
        "tiff" => Some(ImageFormat::Tiff),
        "x-icon" => Some(ImageFormat::Ico),
        _ => None,
    }
}
