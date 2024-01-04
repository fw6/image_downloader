use anyhow::{anyhow, Ok, Result};
use image::ImageFormat;
use reqwest::Response;
use url::Url;

use crate::utils::{get_content_type, mime_to_image_format};

pub async fn get_image_by_url(url: &Url) -> Result<(ImageFormat, Response)> {
    // 发送请求拿到的response
    let res = reqwest::get(url.as_str()).await?;
    let headers = res.headers();

    // 获取response的content-type
    let content_type = get_content_type(&headers)?;
    // 根据response的content-type判断图片格式
    let image_format = mime_to_image_format(content_type.clone()).ok_or(anyhow!(
        "Unsupported image format: {:?}",
        content_type.subtype()
    ))?;

    Ok((image_format, res))
}

#[cfg(test)]
mod tests {
    use super::get_image_by_url;
    use url::Url;

    #[tokio::test]
    async fn test_get_image_by_url() {
        let url = Url::parse("https://www.baidu.com/img/flexible/logo/pc/result.png").unwrap();
        let res = get_image_by_url(&url).await.unwrap();

        assert_eq!(res.0, image::ImageFormat::Png);

        let file_ext = res.0.extensions_str().first();
        assert!(file_ext.is_some());
    }
}
