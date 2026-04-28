use mongodb::{bson::{doc, Document}, Client};
use futures::TryStreamExt;
use crate::openai::chat::ChatOpenAI;
use crate::openai::embed::EmbedOpenAI;
use std::env;

pub(crate) async fn handler_embedding() -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    let llm = EmbedOpenAI::new("text-embedding-3-small");
    let question = "I want to stay in a place that's warm and friendly, \
            and not too far from resturants, can you recommend a place? \
            Include a reason as to why you've chosen your selection.";

    let response = llm
        .with_dimensions(1536)
        .embed_content(question)
        .await
        .expect("Failed to get embedding");
    
    let embeddings = response.data[0].embedding.clone();

    let pipeline  = vec! [
        doc! {
            "$vectorSearch": doc! {
            "queryVector": embeddings,
            "path": "text_embeddings",
            "numCandidates": 150,
            "index": "vector_index",
            "limit": 5
        }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "name": 1,
                "accommodates": 1,
                "address.street": 1,
                "summary": 1,
                "description": 1,
                "neighborhood_overview": 1,
                "notes": 1,
                "score": doc! { "$meta": "vectorSearchScore" },
            }
        }
    ];

    let coll = client.database("airbnb_dataset").collection::<Document>("listings_reviews");
    let mut results = coll.aggregate(pipeline).await?;
    let mut search_results = Vec::new();
    while let Some(result) = results.try_next().await? {
        search_results.push(result);
    }

    let llm = ChatOpenAI::new("gpt-4.1-mini");

    let system_prompt = "You are a airbnb listing recommendation system.";
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