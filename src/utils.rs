use anyhow::{anyhow, Ok, Result};
use console::Style;
use futures_util::StreamExt;
use image::{load_from_memory, ImageFormat};
use mime::Mime;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::header::CONTENT_TYPE;
use std::io::Write as _;
use std::path::PathBuf;
use tokio::io::copy;
use url::Url;

pub fn process_error_output(error: Result<()>) -> Result<()> {
    if let Err(e) = error {
        let stderr = std::io::stderr();
        let mut stderr = stderr.lock();

        if atty::is(atty::Stream::Stderr) {
            let s = Style::new().red();
            write!(stderr, "{}", s.apply_to(format!("{:?}", e)))?;
        } else {
            write!(stderr, "{:?}", e)?;
        }
    }

    Ok(())
}

pub async fn download_image(
    url: &Url,
    formats: &Vec<ImageFormat>,
    filename: &str,
    output: &PathBuf,
) -> Result<()> {
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

    // 获取response的body, 即图片的二进制数据流
    let mut bytes_stream = res.bytes_stream();
    // 用于存储图片的二进制数据
    let mut dest_bytes: Vec<u8> = Vec::new();

    let file_ext = image_format.extensions_str().first();
    if file_ext.is_none() {
        return Err(anyhow!("Invalid file extension"));
    }

    // 保存的目录名
    let dirname = filename;
    let output_dir = output.to_str().map_or("images", |v| v);
    // 创建个目录
    std::fs::create_dir_all(format!("{}/{}", output_dir, dirname))?;

    let file_path = format!("{}/{}/{}.", output_dir, dirname, filename);

    let mut dest_file = tokio::fs::File::create(file_path.to_owned() + file_ext.unwrap()).await?;

    while let Some(item) = bytes_stream.next().await {
        if let std::result::Result::Ok(bytes) = item {
            let bytes = bytes.to_vec();
            dest_bytes.append(&mut bytes.clone());
            copy(&mut bytes.as_ref(), &mut dest_file).await?;
        }
    }

    let img = load_from_memory(&dest_bytes)?;

    formats.par_iter().all(|format| {
        if image_format.ne(format) {
            if let Some(ext_name) = format.extensions_str().first() {
                let file_path = file_path.to_owned() + ext_name;

                let res = img
                    .save_with_format(&file_path, format.to_owned())
                    .map_err(anyhow::Error::msg);

                if let Err(e) = res {
                    let stderr = std::io::stderr();
                    let mut stderr = stderr.lock();

                    if atty::is(atty::Stream::Stderr) {
                        let s = Style::new().red();
                        write!(stderr, "{}", s.apply_to(format!("{:?}", e))).unwrap();
                    } else {
                        write!(stderr, "{:?}", e).unwrap();
                    }

                    return false;
                }
            }
        }

        true
    });

    Ok(())
}

fn get_content_type(headers: &reqwest::header::HeaderMap) -> Result<mime::Mime> {
    headers
        .get(CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .map(|mime| mime.parse().unwrap_or(mime::APPLICATION_OCTET_STREAM))
        .ok_or_else(|| anyhow!("Failed to get content type"))
}

fn mime_to_image_format(mime: Mime) -> Option<ImageFormat> {
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

// This is to ensure that all progress bar prefixes are aligned.
pub const PROGRESS_PREFIX_LEN: usize = 24;

pub fn default_progress_bar(len: u64) -> indicatif::ProgressBar {
    let pb = indicatif::ProgressBar::new(len);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{prefix:.bold}▕{bar:.magenta}▏{msg}")
            .expect("valid indicatif template")
            .progress_chars("█▓▒░  "),
    );
    pb
}
