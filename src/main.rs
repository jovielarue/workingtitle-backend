use axum::{Router, routing::get};
use mongodb::{Client, Collection};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{db::db_connection, user::User};
mod api;
mod db;
mod user;

#[tokio::main]
async fn main() {
    // create tracing layer -- outputs logs from requests/responses/etc.
    let tracing_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry().with(tracing_layer).init();

    // start application
    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let db_client = db_connection().await;
    axum::serve(listener, app(db_client)).await.unwrap();
}

fn app(client: Client) -> Router {
    let users: Collection<User> = client.database("workingtitle").collection("users");

    // merge routes from other modules with .merge()
    Router::new()
        .with_state(users)
        .route("/", get(index))
        .merge(user::users_router())
        .layer(TraceLayer::new_for_http())
}

async fn index() -> &'static str {
    "hello world"
}
