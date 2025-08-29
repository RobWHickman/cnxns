pub const CREATE_DAILY_SELECTION_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS pi_db.connections.daily_selection (
   date DATE PRIMARY KEY,
   player1_id VARCHAR(80),
   player1_full_name VARCHAR(255),
   player2_id VARCHAR(80),
   player2_full_name VARCHAR(255),
   optimal_distance INT DEFAULT 0,
   created_at_utc TIMESTAMP DEFAULT NOW(),
   updated_at_utc TIMESTAMP DEFAULT NOW(),
   FOREIGN KEY (player1_id) REFERENCES pi_db.connections.players(player_id),
   FOREIGN KEY (player2_id) REFERENCES pi_db.connections.players(player_id),
   CONSTRAINT different_players CHECK (player1_id != player2_id)
);
"#;

pub const GENERATE_DAILY_SELECTION: &str = r#"
WITH used_players AS (
    SELECT player1_id AS player_id FROM pi_db.connections.daily_selection
    UNION
    SELECT player2_id AS player_id FROM pi_db.connections.daily_selection
),
possible_players AS (
    SELECT p.player_id, p.full_name, COUNT(DISTINCT m.match_id) AS n
    FROM pi_db.connections.players p 
    JOIN pi_db.connections.player_stats ps ON ps.player_id = p.player_id
    JOIN pi_db.connections.matches m ON ps.match_id = m.match_id
    WHERE ps.variable = 'mins_played'
      AND m.league_id = '9'
      AND p.player_id NOT IN (SELECT player_id FROM used_players)
    GROUP BY p.player_id, p.full_name
    HAVING COUNT(DISTINCT m.match_id) > 30
),
random_pair AS (
    SELECT player_id, full_name, ROW_NUMBER() OVER (ORDER BY RANDOM()) as rn
    FROM possible_players
),
selected_players AS (
    SELECT 
        MAX(CASE WHEN rn = 1 THEN player_id END) as player1_id,
        MAX(CASE WHEN rn = 1 THEN full_name END) as player1_full_name,
        MAX(CASE WHEN rn = 2 THEN player_id END) as player2_id,
        MAX(CASE WHEN rn = 2 THEN full_name END) as player2_full_name
    FROM random_pair
    WHERE rn <= 2
)
INSERT INTO pi_db.connections.daily_selection (date, player1_id, player1_full_name, player2_id, player2_full_name)
SELECT CURRENT_DATE, player1_id, player1_full_name, player2_id, player2_full_name
FROM selected_players
WHERE NOT EXISTS (
    SELECT 1 FROM pi_db.connections.daily_selection 
    WHERE date = CURRENT_DATE
)
AND player1_id IS NOT NULL 
AND player2_id IS NOT NULL;
"#;

pub const CREATE_TEAMS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS pi_db.connections.teams (
    team_id VARCHAR(80) PRIMARY KEY,
    team_name VARCHAR(255),
    common_name VARCHAR(255),
    colour1 VARCHAR(50) CHECK (colour1 IN ('red', 'blue', 'green', 'yellow', 'purple', 'orange', 'brown', 'black', 'white') OR colour1 IS NULL),
    colour2 VARCHAR(50) CHECK (colour2 IN ('red', 'blue', 'green', 'yellow', 'purple', 'orange', 'brown', 'black', 'white') OR colour2 IS NULL),
    created_at_utc TIMESTAMP DEFAULT NOW(),
    updated_at_utc TIMESTAMP DEFAULT NOW()
);
"#;

pub const REFRESH_TEAMS_TABLE: &str = r#"
INSERT INTO pi_db.connections.teams (team_id, team_name, common_name, colour1, colour2)
WITH all_teams AS (
    SELECT DISTINCT home_team_id as team_id, home_team_name as team_name FROM pi_db.connections.matches
    UNION
    SELECT DISTINCT away_team_id as team_id, away_team_name as team_name FROM pi_db.connections.matches
),
first_teams AS (
    SELECT team_id, MIN(team_name) as team_name
    FROM all_teams
    GROUP BY team_id
)
SELECT team_id, team_name, NULL, NULL, NULL
FROM first_teams
ON CONFLICT (team_id) DO UPDATE SET
    team_name = EXCLUDED.team_name,
    updated_at_utc = NOW();
"#;

pub const UPDATE_TEAM_COLORS: &str = r#"
UPDATE pi_db.connections.teams 
SET colour1 = $1, colour2 = $2, updated_at_utc = NOW() 
WHERE team_id = $3
"#;
