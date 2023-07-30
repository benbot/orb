pub mod schema;
mod sites;

use axum::{
    Router, routing::get,
};
use diesel::ExpressionMethods;

#[tokio::main]
async fn main() {
    let runtime = orb_runtime::Runtime::new();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
