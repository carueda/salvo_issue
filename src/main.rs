use salvo::oapi::extract::*;
use salvo::prelude::*;

#[tokio::main]
pub async fn main() {
    let router = Router::new().push(
        Router::with_path("/foo").get(get_foo),
    );

    let doc = OpenApi::new("foo api", "0.0.1").merge_router(&router);

    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("/"));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[endpoint(
    parameters(
        ("max_positions", description = "Maximum number of last positions to report"),
    )
)]
async fn get_foo(
    max_positions: QueryParam<usize, false>,
) -> String {
    format!(
        "max_positions = {}",
        max_positions.into_inner().unwrap_or(0),
    )
}
