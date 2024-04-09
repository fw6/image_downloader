extern crate console_error_panic_hook;
// use anyhow::Result;
// use imagekit::avatar::AvatarBuilder;
// use js_sys::Promise;
// use log::{info, Level};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// fn convert_level(level: LogLevel) -> Level {
//     match level {
//         LogLevel::Trace => Level::Trace,
//         LogLevel::Debug => Level::Debug,
//         LogLevel::Info => Level::Info,
//         LogLevel::Warn => Level::Warn,
//         LogLevel::Error => Level::Error,
//     }
// }

#[wasm_bindgen]
pub struct ImagekitClient;

// #[wasm_bindgen(static_method_of = ImagekitClient)]
// pub fn get_avatar() {
//     info!("Hello from Rust!");

//     // Ok(())
//     // console_error_panic_hook::set_once();
//     // let promise = Promise::new(&mut |resolve, reject| {
//     //     resolve.call1(&JsValue::NULL, &JsValue::from_str("Hello, World!"))
//     // });
//     // promise
// }

// pub struct ImagekitClient(Rc<JsonClient>);
