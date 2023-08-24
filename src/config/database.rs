use mongodb::{Client, options::ClientOptions, Database};

pub async fn dbconnect() -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse("mongodb://root:123456@db:27017").await?;

    client_options.app_name = Some("My App".to_string());
    let client = Client::with_options(client_options)?;
    let db = client.database("crab_db");

    Ok(db)
}