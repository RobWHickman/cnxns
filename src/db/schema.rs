pub const CREATE_TABLES_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS league_seasons (
    league_id INT,
    season_id VARCHAR(255),
    league_schedule_url VARCHAR(255),
    number_matches INT,
    scraped BOOLEAN,
    PRIMARY KEY (league_id, season_id)
);

CREATE TABLE IF NOT EXISTS matches (
    league_id INT,
    season_id VARCHAR(255),
    match_id VARCHAR(255),
    home_team_id VARCHAR(255),
    away_team_id VARCHAR(255),
    match_date DATE,
    data_count INT,
    PRIMARY KEY (league_id, season_id, match_id)
);
"#;
