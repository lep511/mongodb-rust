use mongodb::{Client, Collection};
use tokio::fs::File;
use tokio::io::AsyncReadExt; // Trait for read_to_string
use crate::utils::Listing;
use std::error::Error;
use log::{info, error};
use std::env;

/// Reads a JSON file asynchronously using tokio and deserializes its content
/// into a vector of Item structs.
///
/// # Arguments
///
/// * `file_path` - The path to the JSON file.
///
/// # Returns
///
/// A `Result` containing a `Vec<Listing>` if successful, or a `Box<dyn Error>`
/// if an error occurred during file reading or deserialization.
async fn read_json_file(file_path: &str) -> Result<Vec<Listing>, Box<dyn Error>> {
    // Open the file asynchronously using tokio
    let mut file = File::open(file_path).await?;

    // Read the entire file content asynchronously into a string
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).await?;

    // Deserialize the JSON string into a vector of Listing structs
    let data: Vec<Listing> = serde_json::from_str(&json_string)?;

    Ok(data)
}

pub(crate) async fn handler_database(
    file_path: &str
) -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    // Get a handle on the movies collection
    let database = client.database("airbnb_dataset");
    let collection: Collection<Listing> = database.collection("listings_reviews");

    let contents: Vec<Listing> = match read_json_file(file_path).await {
        Ok(items) => {
            // Process the data if reading was successful
            info!("Successfully read {} items:", items.len());
            items
        }
        Err(e) => {
            // Handle the error if reading failed
            error!("Error reading or parsing JSON file: {}", e);
            return Ok(());
        }
    };

    let insert_news_result = collection.insert_many(&contents).await?;   
    info!("Total of documents inserted: {}", insert_news_result.inserted_ids.len());

    Ok(())
}

