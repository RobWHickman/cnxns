use crate::app::data_types::Player;
use crate::app::psql::daily_players::GET_DAILY_PLAYERS;
use crate::app::psql::search_players::SEARCH_PLAYERS_BY_NAME;
use chrono::Local;
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub async fn get_challenge_players() -> Result<Vec<Player>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let today_date = Local::now().date_naive();
    let rows = client.query(GET_DAILY_PLAYERS, &[&today_date]).await?;

    if rows.len() != 1 {
        return Err(format!("Expected 1 row, found {}", rows.len()).into());
    }

    let row = &rows[0];
    let date: Option<chrono::NaiveDate> = row.get("date");
    let player1_id: Option<String> = row.get("player1_id");
    let player1_name: Option<String> = row.get("player1_full_name");
    let player2_id: Option<String> = row.get("player2_id");
    let player2_name: Option<String> = row.get("player2_full_name");

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

pub async fn search_players_by_name(
    query: &str,
) -> Result<Vec<Player>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let search_query = format!("%{}%", query.to_lowercase());
    let rows = client
        .query(SEARCH_PLAYERS_BY_NAME, &[&search_query])
        .await?;

    let players: Vec<Player> = rows
        .iter()
        .map(|row| Player {
            player_id: row.get("player_id"),
            player_name: row.get("full_name"),
        })
        .collect();

    Ok(players)
}
