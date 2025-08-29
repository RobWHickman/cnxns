use cnxns::app::psql::app_schema::{CREATE_DAILY_SELECTION_TABLE, GENERATE_DAILY_SELECTION};
use cnxns::app::team_data::refresh_teams_table;
use dotenv::dotenv;
use postgres::{Client as PgClient, NoTls};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("PI_DB_LOCAL").expect("DATABASE_URL must be set");
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
