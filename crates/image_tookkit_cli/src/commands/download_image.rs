use anyhow::{anyhow, Ok};
use console::Style;
use futures_util::StreamExt;
use image::{self, load_from_memory, DynamicImage, ImageFormat};
use image_toolkit::download::get_image_by_url;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Write as _;
use url::Url;

use super::RunArgs;

async fn download_image(url: &Url, filename: &str) -> anyhow::Result<(ImageFormat, DynamicImage)> {
    let (image_format, res) = get_image_by_url(&url).await?;
    let file_ext = image_format
        .extensions_str()
        .first()
        .ok_or(anyhow!("Unsupported image format: {:?}", image_format))?;

    let filename = format!("{}.{}", filename, file_ext);
    let mut res_bytes = Vec::new();

    let mut stream = res.bytes_stream();
    let mut file = tokio::fs::File::create(filename).await?;

    while let Some(item) = stream.next().await {
        if let std::result::Result::Ok(bytes) = item {
            let bytes = bytes.to_vec();
            res_bytes.append(&mut bytes.clone());
            tokio::io::copy(&mut bytes.as_ref(), &mut file).await?;
        }
    }

    let img = load_from_memory(&res_bytes)?;

    Ok((image_format, img))
}

pub async fn run(args: RunArgs) -> anyhow::Result<()> {
    let url = args.url;
    let formats = args.formats;
    let filename = args.filename;

    if let (Some(url), Some(formats)) = (url, formats) {
        let (image_format, image) = download_image(&url, &filename).await?;

        formats.par_iter().all(|format| {
            if image_format.ne(format) {
                if let Some(ext_name) = format.extensions_str().first() {
                    let file_path = format!("{}.{}", filename, ext_name);

                    let res = image
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
    }

    Ok(())
}
