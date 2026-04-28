use env_logger::Env;
mod create_database;
mod query;
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

    match query::handler_query().await {
        Ok(_) => (),
        Err(e) => error!("Error: {}", e),
    }

}
