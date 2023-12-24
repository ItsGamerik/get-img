use std::env;
use log::{info, error};
use simple_logger::SimpleLogger;

// #[async_trait]
// impl Handler for EventHandler {
//
// }

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_colors(true).init().unwrap();
    let token: String = match env::var("DISCORD_TOKEN") {
        Ok(token) => {
            info!("token found in env!");
            token
        },
        Err(e) => {
            error!("no discord token found in env: {e}");
            return;
        }
    };
}