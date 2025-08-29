use postgres::Client as PgClient;
use reqwest::blocking::Client as APIClient;
use serde_json::Value;
use std::thread::sleep;
use std::{env, time::Duration};

const PROBLEMATIC_MATCHES: &[&str] = &["19bad36c", "93a55635", "7110621d", "8d12dd69", "ff278feb"];

struct MatchInfo {
    match_id: String,
    home_team_id: String,
    home_team_name: String,
    away_team_id: String,
    away_team_name: String,
}

pub fn get_match_stats(
    db_client: &mut PgClient,
    config: &toml::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY")?;
    let api_client = APIClient::new();

    let matches = get_match_info(db_client)?;

    for match_info in matches {
        if PROBLEMATIC_MATCHES.contains(&match_info.match_id.as_str()) {
            continue;
        }
        sleep(Duration::from_millis(
            config["fbref_api"]["rate_limit_ms"].as_integer().unwrap() as u64,
        ));
        println!(
            "Requesting match: https://fbref.com/en/matches/{}",
            match_info.match_id
        );
        let match_data = request_match_players(&api_client, &api_key, &match_info.match_id)
            .map_err(|e| format!("Failed to request match {}: {}", match_info.match_id, e))?;
        println!("Got data for match: {}", match_info.match_id);

        process_match_data(&match_data, &match_info, db_client)
            .map_err(|e| format!("Failed to process match {}: {}", match_info.match_id, e))?;
        println!("Processed match: {}", match_info.match_id);
    }
    Ok(())
}

fn get_match_info(db_client: &mut PgClient) -> Result<Vec<MatchInfo>, Box<dyn std::error::Error>> {
    let rows: Vec<postgres::Row> = db_client.query(
        "SELECT match_id, home_team_id, home_team_name, away_team_id, away_team_name 
        FROM pi_db.pi_db.connections.matches 
        WHERE match_id NOT IN (SELECT DISTINCT match_id FROM pi_db.pi_db.connections.player_stats)",
        &[],
    )?;

    let matches = rows
        .iter()
        .map(|row| MatchInfo {
            match_id: row.get(0),
            home_team_id: row.get(1),
            home_team_name: row.get(2),
            away_team_id: row.get(3),
            away_team_name: row.get(4),
        })
        .collect();
    Ok(matches)
}

fn request_match_players(
    api_client: &APIClient,
    api_key: &str,
    match_id: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    println!("Making API request for match: {}", match_id);
    let response = match api_client
        .get("https://fbrapi.com/all-players-match-stats")
        .header("X-API-Key", api_key)
        .query(&[("match_id", match_id)])
        .send()
    {
        Ok(resp) => resp,
        Err(e) => {
            println!("HTTP request failed for match {}: {}", match_id, e);
            return Err(format!("HTTP request failed for match {}: {}", match_id, e).into());
        }
    };

    println!("Got response with status: {}", response.status());
    if !response.status().is_success() {
        let status = response.status();
        let error_body = response
            .text()
            .unwrap_or_else(|_| "Could not read error body".to_string());
        println!("API Error Body: {}", error_body);
        return Err(format!(
            "API request failed with status: {} - Body: {}",
            status, error_body
        )
        .into());
    }
    println!("Parsing JSON response...");
    let json: Value = response.json()?;
    println!("Successfully parsed JSON for match: {}", match_id);
    Ok(json)
}

fn persist_player_data(
    db_client: &mut PgClient,
    player_data: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let player_id = player_data["meta_data"]["player_id"]
        .as_str()
        .ok_or("Missing player_id in meta_data")?;
    let full_name = player_data["meta_data"]["player_name"]
        .as_str()
        .ok_or("Missing player_name in meta_data")?;
    let nationality: Option<String> = player_data["meta_data"]["player_country_code"]
        .as_str()
        .map(|s| s.to_string());

    db_client.execute(
        "INSERT INTO pi_db.pi_db.connections.players (player_id, full_name, nationality, current_club, active) 
         VALUES ($1, $2, $3, $4, $5) 
         ON CONFLICT (player_id) DO NOTHING",
        &[
            &player_id,
            &full_name,
            &nationality,
            &None::<String>,
            &false,
        ],
    )?;

    Ok(())
}

fn persist_match_stats(
    db_client: &mut PgClient,
    match_info: &MatchInfo,
    team_name: &str,
    player_data: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let player_id = player_data["meta_data"]["player_id"]
        .as_str()
        .ok_or("Missing player_id in meta_data")?;

    let team_id = if team_name == match_info.home_team_name {
        &match_info.home_team_id
    } else if team_name == match_info.away_team_name {
        &match_info.away_team_id
    } else {
        return Err(format!("Team name '{}' not found in match", team_name).into());
    };

    let summary = &player_data["stats"]["summary"];
    let mins_played: f64 = summary["min"]
        .as_str()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0.0);
    let goals: f64 = summary["gls"].as_f64().unwrap_or(0.0);
    let assists: f64 = summary["ast"].as_f64().unwrap_or(0.0);

    let stats = [
        ("mins_played", mins_played),
        ("goals", goals),
        ("assists", assists),
    ];

    for (variable, value) in stats {
        db_client.execute(
            "INSERT INTO pi_db.pi_db.connections.player_stats (match_id, team_id, player_id, variable, value) 
             VALUES ($1, $2, $3, $4, $5) 
             ON CONFLICT DO NOTHING",
            &[
                &match_info.match_id,
                &team_id,
                &player_id,
                &variable,
                &value,
            ],
        )?;
    }
    Ok(())
}

fn process_match_data(
    match_data: &Value,
    match_info: &MatchInfo,
    db_client: &mut PgClient,
) -> Result<(), Box<dyn std::error::Error>> {
    for team in match_data["data"]
        .as_array()
        .ok_or("Missing 'data' array in match_data")?
    {
        let team_name = team["team_name"]
            .as_str()
            .ok_or("Missing team_name in team data")?;

        for player in team["players"]
            .as_array()
            .ok_or("Missing 'players' array in team data")?
        {
            persist_player_data(db_client, player)?;
            persist_match_stats(db_client, match_info, team_name, player)?;
        }
    }
    Ok(())
}
