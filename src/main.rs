use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use mongodb::{Client, Collection};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    auth::{authorize, protected_test},
    db::db_connection,
    events::models::Event,
    tickets::models::Ticket,
    users::models::User,
};
mod api;
mod auth;
mod db;
mod events;
mod tickets;
mod users;
mod utility;

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

struct AppState {
    users: Collection<User>,
    events: Collection<Event>,
    tickets: Collection<Ticket>,
}

fn app(client: Client) -> Router {
    let app_state = Arc::new(AppState {
        users: client.database("workingtitle").collection("users"),
        events: client.database("workingtitle").collection("events"),
        tickets: client.database("workingtitle").collection("tickets"),
    });

    // merge routes from other modules with .merge()
    // pass AppState to other routers with Arc::clone()
    Router::new()
        .with_state(Arc::clone(&app_state))
        .route("/authorize", post(authorize))
        .route("/protected", get(protected_test))
        .merge(users::user_router(Arc::clone(&app_state)))
        .merge(events::event_router(Arc::clone(&app_state)))
        .merge(tickets::ticket_router(Arc::clone(&app_state)))
        .layer(TraceLayer::new_for_http())
}
