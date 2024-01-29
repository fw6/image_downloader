use imagekit::JuliafatouBuilder;
use salvo::oapi::endpoint;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

// TODO
// 排除OPENAPI传递的部分参数(take_time、output_file)
#[endpoint(parameters(JuliafatouBuilder), responses(
    (status_code = 200, description = "success response")
))]
pub async fn juliafatou(req: &mut Request) -> String {
    let query = req.parse_queries::<User>().unwrap();
    // query.id;

    // query.
    // format!("Hello, {}!", name.as_deref().unwrap_or("World"))
    println!("{:?}", query);
    format!("Hello, {}!", "World")
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    id: usize,
    name: String,
}
