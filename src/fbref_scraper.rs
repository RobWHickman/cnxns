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
    Ok(())
}
