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
