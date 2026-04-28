use mongodb::{Client, Collection};
use mongodb::bson::{doc, from_document, Document};
use futures::TryStreamExt;
use crate::utils::Listing;
use crate::openai::chat::ChatOpenAI;
use crate::openai::embed::EmbedOpenAI;
use serde::Deserialize;
use log::error;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct ResultLocation {
    name: String,
    accommodates: i32,
    address: Address,
    number_of_reviews: Option<i32>,
    bedrooms: f32,
    space: Option<String>,
    average_review_score: Option<f64>,
    combined_score: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Address {
    country: String,
    street: String,
    market: String,
}

pub(crate) async fn handler_filters() -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    // Get a handle on the airbnb collection
    let database = client.database("airbnb_dataset");
    let collection: Collection<Listing> = database.collection("listings_reviews");

    let llm = EmbedOpenAI::new("text-embedding-3-small");
    let question = "I want to stay in a place that's warm and friendly, \
        and not too far from resturants, can you recommend a place? \
        Include a reason as to why you've chosen your selection";
    
    let response = llm
        .with_dimensions(1536)
        .embed_content(question)
        .await
        .expect("Failed to get embedding");
    
    let embeddings = response.data[0].embedding.clone();

    // Specifying the metadata field to limit documents on
    let search_path = "address.country";

    // Create a match stage
    let match_stage = doc! {
        "$match": {
            search_path: { "$regex": "United States" },
            "accommodates": { "$gt": 1, "$lt": 8 }
        }
    };

    // Boosting Search
    let review_average_stage = create_review_average_stage();
    let weighting_stage = create_weighting_stage();
    let sorting_stage = create_sorting_stage();

    let pipeline = vec! [
        doc! {
            "$vectorSearch": doc! {
            "queryVector": embeddings,
            "path": "text_embeddings",
            "numCandidates": 150,
            "index": "vector_index_with_filter",
            "limit": 20,
            "filter": {
                "$and": [
                    {"accommodates": {"$gte": 2}},
                    {"bedrooms": {"$lte": 7}}
                ]
            }
        }},
        doc! {
            "$project": doc! {
                "_id": 0,
                "name": 1,
                "accommodates": 1,
                "address.country": 1,
                "address.street": 1,
                "address.market": 1,
                "number_of_reviews": 1,
                "bedrooms": 1,
                "space": 1,
                "average_review_score": review_average_stage,
            }
        },
        doc! {
            "$addFields": doc! {
                "combined_score": weighting_stage
            }
        },
        match_stage,
        sorting_stage
    ];

    let mut results = collection.aggregate(pipeline).await?;
    let mut search_results = Vec::new();

    while let Some(result) = results.try_next().await? {
        match from_document::<ResultLocation>(result) {
            Ok(listing) => {
                println!("Found: {} | \
                    average_review_score - {} | \
                    combined_score - {} | number_of_reviews - {}",
                    listing.name,
                    listing.average_review_score.unwrap_or(0.0),
                    listing.combined_score.unwrap_or(0.0),
                    listing.number_of_reviews.unwrap_or(0)
                ); 
                search_results.push(listing);
            }
            Err(e) => {
                error!("Error converting document: {}", e);
            }
        }
    }

    let llm = ChatOpenAI::new("gpt-4.1-mini");

    let system_prompt = "You are a airbnb listing recommendation system. \
                Always reply with number of rooms and if have accommodation";

    let prompt = format!("Answer this user query with the context: {} \n \
        If the context is empty, the response is that you are missing data. \
        \n \
        Context:\n{:?}",
        question,
        search_results);

    let response = llm
        .with_max_tokens(4096)
        .with_system_prompt(system_prompt)
        .invoke(&prompt)
        .await
        .expect("Failed to get response");

    match response.choices {
        Some(candidates) => {
            candidates.iter()
                .filter_map(|candidate| candidate
                    .message.as_ref()?
                    .content.as_ref()
                ).for_each(|content| println!("{}", content));
        }
        None => println!("No response choices available"),
    };

    Ok(())
}

fn create_review_average_stage() -> Document {
    doc! {
        "$divide": [
            {
                "$add": [
                    "$review_scores.review_scores_accuracy",
                    "$review_scores.review_scores_cleanliness",
                    "$review_scores.review_scores_checkin",
                    "$review_scores.review_scores_communication",
                    "$review_scores.review_scores_location",
                    "$review_scores.review_scores_value"
                ]
            },
            6 // Divide by the number of review score types
        ]
    }
}

fn create_weighting_stage() -> Document {
    doc! {
        "$add": [
            {
                // Weighted average review score
                "$multiply": ["$average_review_score", 0.9]
            },
            {
                // Weighted review count boost
                "$multiply": ["$number_of_reviews", 0.1]
            }
        ]
    }
}

fn create_sorting_stage() -> Document {
    doc! {
        "$sort": {
            // Descending order to boost higher combined scores
            "combined_score": -1
        }
    }
}