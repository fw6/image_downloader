use log::info;
use napi_derive_ohos::napi;

#[napi(js_name = "add")]
pub fn add(first: i32, second: i32) -> i32 {
    ohos_hilog::init_once(
        ohos_hilog::Config::default()
            .with_max_level(log::LevelFilter::Trace)
            .with_tag("imagekit")
            .with_filter(
                ohos_hilog::FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build(),
            ),
    );

    info!("start add");

    first + second
}
