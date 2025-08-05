use cnxns::db::schema::CREATE_TABLES_SQL;
use cnxns::scrape::scraping::get_new_fixtures;
use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::env;
use std::fs;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&database_url, NoTls).unwrap();
    client.batch_execute(CREATE_TABLES_SQL).unwrap();

    let content = fs::read_to_string("./config.toml").expect("Failed to read config");
    let config: toml::Value = toml::from_str(&content).expect("Failed to parse config");

    get_new_fixtures(client, &config)
}
