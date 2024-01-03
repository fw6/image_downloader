use std::path::PathBuf;

use anyhow::{anyhow, Ok, Result};
use clap::{Parser, Subcommand};
use image::ImageFormat;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
};
use serde_json::json;
use serde_json_lodash::get;
use tokio::task::JoinSet;
use url::Url;

use crate::{
    cli::{parse_image_formats, parse_path, parse_url},
    download_image, process_error_output,
    utils::{default_progress_bar, PROGRESS_PREFIX_LEN},
    Env,
};

/// Diff two http requests and compare the difference of the responses
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

    /// Download user avatar from url
    Rank2023(Rank2023Args),
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

#[derive(Parser, Debug, Clone)]
struct Rank2023Args {
    /// The env of the ranking url. support dev、test、prod
    #[clap(short, long)]
    env: Env,

    /// The image format to download, support multiple formats.
    /// Supported formats: png (png)、jpeg (jpg)、webp (webp)、gif (gif)、avif (avif)、bmp (bmp)、tiff (tiff)、ico (ico)
    #[clap(short, long, value_parser = parse_image_formats, value_delimiter = ' ', num_args = 1..)]
    formats: Option<Vec<ImageFormat>>,

    /// The output path of the image
    /// e.g. -o /Users/feng.w/Desktop
    #[clap(short, long, value_name = "PATH", value_parser = parse_path, default_value = "images")]
    output: PathBuf,
}

async fn run(args: RunArgs) -> Result<()> {
    let url = args.url;
    let formats = args.formats;

    if let (Some(url), Some(formats)) = (url, formats) {
        download_image(&url, &formats, &args.filename, &PathBuf::from("images")).await?;
    }

    Ok(())
}

async fn run_rank_2023(args: Rank2023Args) -> Result<()> {
    let env = args.env;
    let formats = args
        .formats
        .map_or_else(|| Ok(vec![ImageFormat::Png]), Ok)?;
    let output = args.output;

    let url = match env {
        Env::Dev => {
            "https://gateway.m.fws.qa.nt.ctripcorp.com"
            // "https://api.spotify.com/v1/search"
        }
        Env::Test => "https://gateway.m.uat.qa.nt.ctripcorp.com",
        Env::Prod => "https://m.ctrip.com",
    };

    let mut set = JoinSet::new();
    let client = Client::new();
    let pb = default_progress_bar(6);
    pb.set_prefix(format!(
        "{:width$}",
        "Fetch user avatars",
        width = PROGRESS_PREFIX_LEN
    ));

    for tab_id in (1..7).into_iter() {
        let client = client.clone();
        let url = url.to_owned();

        set.spawn(async move {
            let result = get_avatars_by_tab_id_from_api(&tab_id, &client, &url).await;
            result
        });
    }

    let mut avatars = Vec::new();

    while let Some(result) = set.join_next().await {
        let result = result??;
        avatars.append(&mut result.clone());
        pb.inc(1);
    }

    pb.finish();
    println!("\nTotal avatars: {}\n", avatars.len());

    avatars.dedup_by_key(|item| item.1.clone());

    let mut set = JoinSet::new();
    let pb = default_progress_bar(avatars.len() as u64);
    pb.set_prefix(format!(
        "{:width$}",
        "Download avatars",
        width = PROGRESS_PREFIX_LEN
    ));

    for item in avatars {
        let avatar_url = item.0.to_owned();
        let nick_name = item.1.to_owned();
        let formats = formats.to_owned();
        let output = output.to_owned();

        set.spawn(async move {
            download_image(&Url::parse(&avatar_url)?, &formats, &nick_name, &output).await
        });
    }

    while let Some(result) = set.join_next().await {
        result??;
        pb.inc(1);
    }

    pb.finish();
    Ok(())
}

async fn get_avatars_by_tab_id_from_api(
    tab_id: &i32,
    client: &Client,
    url: &str,
) -> Result<Vec<(String, String)>> {
    let resp = client
        .post(format!(
            "{}{}",
            url, "/restapi/soa2/20725/json/getAnnualRankingList"
        ))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .json(&json!({
            "tabId": tab_id
        }))
        .send()
        .await?
        .text()
        .await?;
    let resp = serde_json::from_str::<serde_json::Value>(&resp)
        .map_err(|err| anyhow!(format!("Failed to decode response: {}", err)))?;

    let resp_list = get(resp, json!(["list"]), json!([]));
    if let Some(list) = resp_list.as_array() {
        return Ok(list
            .par_iter()
            .flat_map_iter(|item| {
                if let Some(_item) = item.as_object() {
                    let subgroup_list =
                        get(item.to_owned(), json!(["annualRankingList"]), json!([]));

                    if let Some(list) = subgroup_list.as_array() {
                        let user_avatar = list
                            .par_iter()
                            .flat_map_iter(|item| {
                                if let Some(item) = item.as_object() {
                                    let avatar_url = item.get("headPhoto");
                                    let nick_name = item.get("nickName");

                                    if let (Some(avatar_url), Some(nick_name)) =
                                        (avatar_url, nick_name)
                                    {
                                        if let (Some(avatar_url), Some(nick_name)) =
                                            (avatar_url.as_str(), nick_name.as_str())
                                        {
                                            let avatar_url = avatar_url.to_owned();
                                            let nick_name = nick_name.to_owned();
                                            return vec![(avatar_url, nick_name)];
                                        }
                                    }
                                }

                                return vec![];
                            })
                            .collect::<Vec<(String, String)>>();

                        return user_avatar;
                    }
                }

                vec![]
            })
            .collect::<Vec<(String, String)>>());
    }

    Ok(vec![])
}

pub async fn handle() -> Result<()> {
    let args = Args::parse();

    let result = match args.action {
        Action::Run(args) => run(args).await,
        Action::Rank2023(args) => run_rank_2023(args).await,
    };

    process_error_output(result)
}
