use std::io::Cursor;

use image::ImageOutputFormat;
use imagekit::avatar::AvatarBuilder;
use salvo::{http::ResBody, oapi::endpoint, prelude::*};

// 排除OPENAPI传递的部分参数(take_time、output_file)
#[endpoint(parameters(AvatarBuilder), responses(
    (status_code = 200, description = "success response")
))]
pub async fn avatar(req: &mut Request, res: &mut Response) {
    let av = req
        .parse_queries::<AvatarBuilder>()
        .expect("parse query params failed");

    let av = av.build();

    if let Ok(av) = av {
        let img = av.draw();
        let mut buffer = Cursor::new(Vec::new());
        let _ = img.write_to(&mut buffer, ImageOutputFormat::Png);

        res.headers_mut()
            .insert("Content-Type", "image/png".parse().unwrap());

        res.body(ResBody::Once(buffer.into_inner().into()));
    } else if let Err(e) = av {
        res.render(
            StatusError::internal_server_error()
                .brief(format!("error when build initial avatar: {}", e)),
        );
    }
}
