use cnxns::app::psql::app_schema::{CREATE_DAILY_SELECTION_TABLE, GENERATE_DAILY_SELECTION};
use dotenv::dotenv;
use postgres::{Client as PgClient, NoTls};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut db_client = PgClient::connect(&database_url, NoTls).unwrap();

    println!("Setting daily challenge...");
    db_client
        .batch_execute(CREATE_DAILY_SELECTION_TABLE)
        .unwrap();
    db_client.batch_execute(GENERATE_DAILY_SELECTION).unwrap();

    Ok(())
}
