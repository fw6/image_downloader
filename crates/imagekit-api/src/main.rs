mod avatar;
mod juliafatou;
use salvo::{logging::Logger, prelude::*};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(Router::with_path("juliafatou.png").get(juliafatou::juliafatou))
        .push(Router::with_path("avatar.png").get(avatar::avatar));

    let doc = OpenApi::new("imagekit api", env!("CARGO_PKG_VERSION")).merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

    let service = Service::new(router).hoop(Logger::new());
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(service).await;
}
