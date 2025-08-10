use chrono::NaiveDate;
use postgres::Client as PgClient;
use reqwest::blocking::Client as APIClient;
use serde_json::Value;
use std::env;
use std::thread::sleep;
use std::time::Duration;

pub fn get_leagues_fixtures(
    db_client: &mut PgClient,
    config: &toml::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let league_configs = config["fbref_ids"]["leagues"].as_array().unwrap();
    let full_season_ids = config["fbref_ids"]["FULL_SEASON_IDS"].as_array().unwrap();

    for league in league_configs {
        let league_id = league["league_id"].as_integer().unwrap().to_string();
        let league_name = league["league_name"].as_str().unwrap();
        let season_ids_raw = league["season_ids"].as_array().unwrap();
        let season_ids = if season_ids_raw.len() == 1 && season_ids_raw[0].as_str() == Some("FULL")
        {
            full_season_ids
        } else {
            season_ids_raw
        };

        for season in season_ids {
            if check_league_status(db_client, league_id.clone(), season.as_str().unwrap())? {
                println!("Skipping: {} for season: {}", league_name, season);
                continue;
            }
            println!("Requesting: {} for season: {}", league_name, season);
            let season_id = season.as_str().unwrap();

            let fixtures = request_fixtures(&league_id, season_id)?;
            persist_league_fixtures(db_client, &league_id, league_name, season_id, &fixtures)?;

            sleep(Duration::from_millis(
                config["fbref_api"]["rate_limit_ms"].as_integer().unwrap() as u64,
            ));
        }
    }
    Ok(())
}

fn check_league_status(
    db_client: &mut PgClient,
    league_id: String,
    season_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let query = "SELECT number_matches, data_count FROM connections.public.league_seasons WHERE league_id = $1 AND season_id = $2";
    let rows = db_client.query(query, &[&league_id, &season_id])?;

    if let Some(row) = rows.first() {
        let number_matches: Option<i32> = row.get("number_matches");
        let data_count: Option<i32> = row.get("data_count");

        if let (Some(num_matches), Some(data_cnt)) = (number_matches, data_count) {
            return Ok(num_matches == data_cnt);
        }
    }

    Ok(false)
}

fn request_fixtures(league_id: &str, season_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY")?;
    let api_client = APIClient::new();

    let response = api_client
        .get("https://fbrapi.com/matches")
        .header("X-API-Key", api_key)
        .query(&[("league_id", league_id), ("season_id", season_id)])
        .send()?;

    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }

    let json: Value = response.json()?;
    Ok(json)
}

fn persist_league_fixtures(
    db_client: &mut PgClient,
    league_id: &str,
    league_name: &str,
    season_id: &str,
    fixture_data: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches = fixture_data["data"].as_array().unwrap();

    db_client.execute(
        "INSERT INTO league_seasons (league_id, league_name, season_id, number_matches) 
         VALUES ($1, $2, $3, $4) ON CONFLICT (league_id, season_id) DO NOTHING",
        &[
            &league_id,
            &league_name,
            &season_id,
            &(matches.len() as i32),
        ],
    )?;

    for match_data in matches {
        let date_str = match_data["date"].as_str().unwrap();
        let parsed_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

        match db_client.execute(
            "INSERT INTO matches (league_id, season_id, match_id, home_team_id, home_team_name, away_team_id, away_team_name, match_date, data_count)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT DO NOTHING",
            &[
                &league_id,
                &season_id,
                &match_data["match_id"].as_str().unwrap(),
                &match_data["home_team_id"].as_str().unwrap(),
                &match_data["home"].as_str().unwrap(),
                &match_data["away_team_id"].as_str().unwrap(),
                &match_data["away"].as_str().unwrap(),
                &parsed_date,
                &0i32
            ]
        ) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to insert match {}: {}", match_data["match_id"].as_str().unwrap_or("unknown"), e);
                println!("Match data: {:#}", match_data);
                return Err(e.into());
            }
        }
    }

    println!(
        "Saved {} matches for {} {}",
        matches.len(),
        league_name,
        season_id
    );
    Ok(())
}
