pub mod errors;
pub mod models;
pub mod routes;

use std::net::SocketAddr;

use axum::Router;

#[tokio::main]
async fn main() {
  let app = Router::new().merge(routes::login::routes());

  let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

  axum::Server::bind(&addr).serve(app.into_make_service()).await;
}

async fn hello() -> String {
  "Hello".to_string()
}
