use axum::{Router, routing::get};
use dotenv::dotenv;
use mongodb::{Client, bson::doc};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod user;

#[tokio::main]
async fn main() {
    // ensure you have the correct .env variable set up (MONGODB_CONNECTION_STRING)
    // format of: mongodb://[username]:[password]@localhost:27017/[workingtitle]?authSource=admin
    dotenv().ok();
    let db_connection_str = std::env::var("MONGODB_CONNECTION_STRING").unwrap();
    println!("{}", db_connection_str);
    let client = Client::with_uri_str(db_connection_str).await.unwrap();
    println!("{:?}", client);
    client
        .database("workingtitle")
        .run_command(doc! { "ping": 1 })
        .await
        .unwrap();
    println!("Successfully connected to MongoDB!");

    // create tracing layer -- outputs logs from requests/responses/etc.
    let tracing_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry().with(tracing_layer).init();

    // route "/" to fn index()
    // merge routes from other modules with .merge()
    let app = Router::new()
        .route("/", get(index))
        .merge(user::users_router())
        .layer(TraceLayer::new_for_http());

    // start application
    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "hello world"
}
