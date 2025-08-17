use crate::app::connection_types::PlayerConnection;
use crate::app::entity_types::{DailyChallenge, Player};
use crate::app::psql::connections::CHECK_PLAYERS_CONNECTED;
use crate::app::psql::daily_players::GET_DAILY_PLAYERS;
use crate::app::psql::search_players::SEARCH_PLAYERS_BY_NAME;
use chrono::Local;
use tokio_postgres::Client;

pub async fn get_challenge_players(
    client: &Client,
) -> Result<DailyChallenge, Box<dyn std::error::Error>> {
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

    Ok(DailyChallenge {
        player1: players[0].clone(),
        player2: players[1].clone(),
        shortest_route: 0,
    })
}

pub async fn search_players_by_name(
    client: &Client,
    query: &str,
) -> Result<Vec<Player>, Box<dyn std::error::Error>> {
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

pub async fn check_player_connection(
    client: &Client,
    player1_id: String,
    player2_id: String,
) -> Result<Option<PlayerConnection>, Box<dyn std::error::Error>> {
    if player1_id.is_empty() || player2_id.is_empty() {
        return Err("Player IDs cannot be empty".into());
    }

    let rows: Vec<postgres::Row> = client
        .query(CHECK_PLAYERS_CONNECTED, &[&player1_id, &player2_id])
        .await?;

    if rows.is_empty() {
        return Ok(None);
    }

    let shared_matches: i64 = rows[0].get("shared_matches");
    let team_id: String = rows[0].get("team_id");

    println!(
        "Found {} shared matches between {} and {} on team {}",
        shared_matches, &player1_id, &player2_id, team_id
    );

    Ok(Some(PlayerConnection {
        player1_id: player1_id,
        player2_id: player2_id,
        matches_together: shared_matches as i32,
        team_id,
    }))
}

pub async fn check_game_completion(
    client: &Client,
    new_player_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let starting_state = get_challenge_players(&*client).await.unwrap();
    let target_player_id = &starting_state.player2.player_id;

    let completion_check = check_player_connection(
        client,
        new_player_id.to_string(),
        target_player_id.to_string(),
    )
    .await?;

    Ok(completion_check.is_some())
}
