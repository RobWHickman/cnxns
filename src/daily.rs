use cnxns::app::psql::app_schema::{CREATE_DAILY_SELECTION_TABLE, GENERATE_DAILY_SELECTION};
use cnxns::app::team_data::refresh_teams_table;
use dotenv::dotenv;
use postgres::{Client as PgClient, NoTls};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = if env::var("DEPLOYMENT").unwrap_or_default() == "local" {
        env::var("LOCALHOST_DB_STRING").expect("LOCALHOST_DB_STRING must be set")
    } else {
        env::var("PI_DB_STRING").expect("PI_DB_STRING must be set")
    };
    let mut db_client = PgClient::connect(&database_url, NoTls).unwrap();

    println!("Refreshing teams table...");
    refresh_teams_table(&mut db_client)?;

    println!("Setting daily challenge...");
    db_client
        .batch_execute(CREATE_DAILY_SELECTION_TABLE)
        .unwrap();
    db_client.batch_execute(GENERATE_DAILY_SELECTION).unwrap();

    Ok(())
}
