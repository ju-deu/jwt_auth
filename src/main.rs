use axum::Router;
use axum::routing::get;




#[tokio::main]
async fn main() {
    // make app
    let app = Router::new()
        .route("/ping", get(|| async { "Hello, World!" }))
    ;

    // serve on 0.0.0.0:8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
