pub const SEARCH_PLAYERS_BY_NAME: &str = r#"
    SELECT DISTINCT player_id, full_name 
    FROM connections.players 
    WHERE LOWER(full_name) LIKE $1 
    LIMIT 10
"#;

pub const GET_PLAYER_CAREER: &str = r#"
WITH player_matches AS (
    SELECT DISTINCT ps.team_id, ps.match_id
    FROM connections.player_stats ps
    WHERE player_id = $1
), player_seasons AS (
    SELECT 
        m.season_id, 
        t.team_name,
        COUNT(DISTINCT pm.match_id) AS match_count
    FROM player_matches pm
    JOIN connections.matches m ON m.match_id = pm.match_id
    JOIN connections.teams t ON t.team_id = pm.team_id
    GROUP BY m.season_id, t.team_name
), team_summary AS (
    SELECT 
        team_name,
        MIN(season_id) AS start_season,
        MAX(season_id) AS end_season,
        SUM(match_count)::INTEGER AS total_matches
    FROM player_seasons
    GROUP BY team_name
)
SELECT 
    team_name AS team,
    LEFT(start_season, 4) || '-' || RIGHT(end_season, 4) AS seasons,
    total_matches AS league_matches
FROM team_summary
ORDER BY start_season, team_name
"#;