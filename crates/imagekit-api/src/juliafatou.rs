use std::io::Cursor;

use imagekit::juliafatou::JuliafatouBuilder;
use salvo::{http::response::ResBody, oapi::endpoint, prelude::*};

// 排除OPENAPI传递的部分参数(take_time、output_file)
#[endpoint(parameters(JuliafatouBuilder), responses(
    (status_code = 200, description = "success response")
))]
pub async fn juliafatou(req: &mut Request, res: &mut Response) {
    let jf: JuliafatouBuilder = req.parse_queries().expect("parse query params failed");
    let jf = jf.build();

    if let Ok(jf) = jf {
        let mut buffer = Cursor::new(Vec::new());

        let _ = jf.save_to_buffer(&mut buffer);
        let vec = buffer.into_inner();

        res.headers_mut()
            .insert("Content-Type", "image/png".parse().unwrap());

        res.body(ResBody::Once(vec.into()));
    } else if let Err(e) = jf {
        res.render(
            StatusError::internal_server_error()
                .brief(format!("error when build juliafatou: {}", e)),
        );
    }
}
