use dotenv::dotenv;
use mongodb::{Client, bson::doc};
pub mod crud;

pub async fn db_connection() -> Client {
    // ensure you have the correct .env variable set up (MONGODB_CONNECTION_STRING)
    // format like so: mongodb://[username]:[password]@localhost:27017/[workingtitle]?authSource=admin
    dotenv().ok();
    let db_connection_str = std::env::var("MONGODB_CONNECTION_STRING").unwrap();

    let client = Client::with_uri_str(db_connection_str).await.unwrap();
    client
        .database("workingtitle")
        .run_command(doc! { "ping": 1 })
        .await
        .unwrap();
    println!("Successfully connected to MongoDB!");

    client
}
