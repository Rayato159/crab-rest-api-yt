pub mod config;
pub mod item;

use std::net::SocketAddr;

use axum::{
    routing::{get, post, patch},
    Router,
    http::Method
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a single route
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(
                    [
                        Method::GET,
                        Method::POST,
                        Method::PATCH,
                        Method::PUT,
                        Method::DELETE,
                    ]
                )
        )
        .route("/", get(|| async { "OK" }))
        .route("/item", post(item::handler::insert_one_item))
        .route("/item", get(item::handler::find_items))
        .route("/:item_id/item", get(item::handler::find_one_item))
        .route("/:item_id/item", patch(item::handler::update_item));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server is starting on: {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
