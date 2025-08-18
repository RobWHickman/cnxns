pub const CREATE_TABLES_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS pi_db.connections.league_seasons (
   league_id VARCHAR(10),
   league_name VARCHAR(255),
   season_id VARCHAR(50),
   number_matches INT,
   data_count INT DEFAULT 0,
   created_at_utc TIMESTAMP DEFAULT NOW(),
   updated_at_utc TIMESTAMP DEFAULT NOW(),
   PRIMARY KEY (league_id, season_id)
);

CREATE TABLE IF NOT EXISTS pi_db.connections.matches (
   league_id VARCHAR(10),
   season_id VARCHAR(50),
   match_id VARCHAR(80),
   home_team_id VARCHAR(80),
   home_team_name VARCHAR(255),
   away_team_id VARCHAR(80),
   away_team_name VARCHAR(255),
   match_date DATE,
   data_count INT DEFAULT 0,
   created_at_utc TIMESTAMP DEFAULT NOW(),
   updated_at_utc TIMESTAMP DEFAULT NOW(),
   PRIMARY KEY (league_id, season_id, match_id),
   FOREIGN KEY (league_id, season_id) REFERENCES pi_db.connections.league_seasons(league_id, season_id)
);

CREATE TABLE IF NOT EXISTS pi_db.connections.players (
   player_id VARCHAR(80) PRIMARY KEY,
   full_name VARCHAR(255),
   nationality VARCHAR(10),
   current_club VARCHAR(80) DEFAULT NULL,
   active BOOLEAN DEFAULT FALSE,
   created_at_utc TIMESTAMP DEFAULT NOW(),
   updated_at_utc TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS pi_db.connections.player_stats (
   match_id VARCHAR(80),
   team_id VARCHAR(80),
   player_id VARCHAR(80),
   variable VARCHAR(255),
   value FLOAT,
   created_at_utc TIMESTAMP DEFAULT NOW(),
   updated_at_utc TIMESTAMP DEFAULT NOW(),
   PRIMARY KEY (match_id, team_id, player_id, variable),
   FOREIGN KEY (player_id) REFERENCES pi_db.connections.players(player_id)
);
"#;
