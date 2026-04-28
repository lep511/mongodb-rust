use mongodb::{Client, Collection};
use mongodb::bson::{doc, from_document, Document};
use futures::TryStreamExt; // Required for try_next() and collect()
use serde::{ Deserialize, Serialize };
use log::{info, error};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ResultLocation {
    name: String,
    career_earnings: i32,
    grand_slam_wins: i32,
    surfaces: Vec<String>,
}

pub(crate) async fn handler_query() -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    // Get a handle on the airbnb collection
    let database = client.database("sports");
    let collection: Collection<Document> = database.collection("sports_data");

    // The query returns athletes that have more than 20 grand slam wins
    let filter = doc! {
        "grand_slam_wins": { "$gt": 20 }
    };

    // Execute the find operation
    let mut cursor = collection.find(filter).await?;

    // Iterate through the results
    info!("Found documents:");
    while let Some(document) = cursor.try_next().await? {
        // Deserialize the document into a ResultLocation struct
        match mongodb::bson::from_document::<ResultLocation>(document) {
            Ok(result_location) => {
                info!("{:?}", result_location);
            }
            Err(e) => {
                error!("Error deserializing document: {}", e);
            }
        }
    }

    Ok(())
}