use crate::app::data_types::Player;
use crate::app::psql::daily_players::GET_DAILY_PLAYERS;
use chrono::Local;
use dotenv::dotenv;
use postgres::{Client as PgClient, NoTls};
use std::env;

pub fn get_challenge_players() -> Result<Vec<Player>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut db_client = PgClient::connect(&database_url, NoTls).unwrap();
    let today_string = Local::now().format("%Y-%m-%d").to_string();
    let rows = db_client.query(GET_DAILY_PLAYERS, &[&today_string])?;

    if rows.len() != 1 {
        return Err(format!("Expected 1 row, found {}", rows.len()).into());
    }

    let row = &rows[0];
    let date: Option<chrono::NaiveDate> = row.get("date");
    let player1_id: Option<String> = row.get("player1_id");
    let player1_name: Option<String> = row.get("player1_name");
    let player2_id: Option<String> = row.get("player2_id");
    let player2_name: Option<String> = row.get("player2_name");

    if date.is_none()
        || player1_id.is_none()
        || player1_name.is_none()
        || player2_id.is_none()
        || player2_name.is_none()
    {
        return Err("Required fields cannot be null".into());
    }

    let players = vec![
        Player {
            player_id: player1_id.unwrap(),
            player_name: player1_name.unwrap(),
        },
        Player {
            player_id: player2_id.unwrap(),
            player_name: player2_name.unwrap(),
        },
    ];
    Ok(players)
}
