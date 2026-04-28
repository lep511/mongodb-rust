use std::ops::Index;
use futures::{TryStreamExt};
use mongodb::{bson::doc, Client, Collection, SearchIndexModel};
use mongodb::SearchIndexType::VectorSearch;
use mongodb::error::Result;
use crate::utils::Listing;
use log::{info, error};
use std::env;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ FILTER INDEX ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) async fn create_filter_index() -> Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    // Get a handle on the airbnb collection
    let database = client.database("airbnb_dataset");
    let collection: Collection<Listing> = database.collection("listings_reviews");

    let index_name = "vector_index_with_filter";
    let mut cursor = collection.list_search_indexes().await?;

    while let Some(index) = cursor.try_next().await? {
        if let Some(index_type) = index.get_str("type").ok() {
            if index_type == "search" {
                if let Some(name) = index.get_str("name").ok() {
                    if name == index_name {
                        info!("Atlas Vector Search index '{}' exist.\n", index_name);
                        return Ok(());
                    }
                }
            }
        }
    }

    let index_definition = doc! {
        "mappings": {
            "dynamic": true,
            "fields": {
                "text_embeddings": {
                    "dimensions": 1536,
                    "similarity": "cosine",
                    "type": "knnVector",
                },
                "accommodates": {
                    "type": "number" // BSON number type
                },
                 "bedrooms": {
                    "type": "number" // BSON number type
                },
            }
        }
    };

    let new_vector_search_index_model = SearchIndexModel::builder()
        .name(index_name.to_string())
        .definition(index_definition)
        .build();

    println!("Creating index...");
    let result = collection.create_search_index(new_vector_search_index_model).await;

    match result {
        Ok(index_name) => {
            println!("New search index named {:?} is building. \
            This may take up to a minute...", 
            index_name);
        }
        Err(e) => {
            eprintln!("Error creating new vector search index: {}", e); // Use eprintln for errors
            return Err(e.into()); // Propagate the error using anyhow or map the error type
        }
    }
    
    Ok(())
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ VECTOR INDEX ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) async fn create_vector_index() -> Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    // Get a handle on the airbnb collection
    let database = client.database("airbnb_dataset");
    let collection: Collection<Listing> = database.collection("listings_reviews");

    let index_name = "vector_index";
    let mut cursor = collection.list_search_indexes().await?;

    while let Some(index) = cursor.try_next().await? {
        if let Some(index_type) = index.get_str("type").ok() {
            if index_type == "vectorSearch" {
                if let Some(name) = index.get_str("name").ok() {
                    if name == index_name {
                        info!("Atlas Vector Search index '{}' exist.\n", index_name);
                        return Ok(());
                    }
                }
            }
        }
    }

    let search_index_def = SearchIndexModel::builder()
        .definition(doc! {
            "dynamic": true,
            "fields": vec! {doc! {
                "type": "vector",
                "path": "text_embeddings",
                "numDimensions": 1536,
                "similarity": "cosine"
            }}
        })
        .name(index_name.to_string())
        .index_type(VectorSearch)
        .build();

    let models = vec![search_index_def];
    let result = collection.create_search_indexes(models).await;
    if let Err(e) = result {
        error!("There was an error creating the search index: {}", e);
        std::process::exit(1)
    } else {
        info!("New search index named {} is building. \
            This may take up to a minute...", 
            result.unwrap().index(0)
        );
    }

    Ok(())
}