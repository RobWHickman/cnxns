use cnxns::app::psql::app_schema::{CREATE_DAILY_SELECTION_TABLE, GENERATE_DAILY_SELECTION};
use cnxns::db::api::fixtures::get_leagues_fixtures;
use cnxns::db::api::generate_key::generate_api_key;
use cnxns::db::api::matches::get_match_stats;
use cnxns::db::psql::schema::CREATE_TABLES_SQL;
use cnxns::db::psql::update_scrape_status::CHECK_SCRAPING_COUNTS;
use dotenv::dotenv;
use postgres::{Client as PgClient, NoTls};
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let content: String = fs::read_to_string("./config.toml").expect("Failed to read config");
    let config: toml::Value = toml::from_str(&content).expect("Failed to parse config");

    _ = fill_db(config);

    Ok(())
}

fn fill_db(config: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
    if env::var("API_KEY").is_err() {
        let _ = generate_api_key();
        return Ok(());
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut db_client = PgClient::connect(&database_url, NoTls).unwrap();
    db_client.batch_execute(CREATE_TABLES_SQL).unwrap();
    db_client.batch_execute(CHECK_SCRAPING_COUNTS).unwrap();
    db_client
        .batch_execute(CREATE_DAILY_SELECTION_TABLE)
        .unwrap();
    db_client.batch_execute(GENERATE_DAILY_SELECTION).unwrap();

    println!("Starting fixtures...");
    get_leagues_fixtures(&mut db_client, &config)?;
    println!("Fixtures complete, starting match stats...");
    get_match_stats(&mut db_client, &config)?;
    println!("Match stats complete");

    Ok(())
}
