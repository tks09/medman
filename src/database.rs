use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn init_db() -> mongodb::Database {
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    client.database("medman")
}
