#[tokio::main]
async fn main() {
    
    // load .env file
    dotenvy::dotenv().expect("Failed to load .env file");

    // start logger (tracing)
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server application...");
}
