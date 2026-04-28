use env_logger::Env;
mod create_database;
mod create_index;
mod embedding;
mod filters;
mod utils;
mod openai;
use log::error;

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // match create_database::handler_database("records.json").await {
    //     Ok(_) => (),
    //     Err(e) => error!("Error: {}", e),
    // }

    // match create_index::create_vector_index().await {
    //     Ok(_) => (),
    //     Err(e) => error!("Error: {}", e),
    // }

    // match create_index::create_filter_index().await {
    //     Ok(_) => (),
    //     Err(e) => error!("Error: {}", e),
    // }

    // match embedding::handler_embedding().await {
    //     Ok(_) => (),
    //     Err(e) => error!("Error: {}", e),
    // }

    match filters::handler_filters().await {
        Ok(_) => (),
        Err(e) => error!("Error: {}", e),
    }
    
}
