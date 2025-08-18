pub const CHECK_SCRAPING_COUNTS: &str = r#"
WITH scraped_matches AS (
    SELECT 
        count(*) as original_count,
        count(*) / 3 as count_divided_by_3,
        count(*) % 3 = 0 as is_integer_division,
        m.match_id
    FROM pi_db.connections.matches m 
    JOIN pi_db.connections.player_stats ps
        ON ps.match_id = m.match_id 
    GROUP BY m.match_id
)
UPDATE pi_db.connections.matches 
SET data_count = 1,
    updated_at_utc = NOW()
FROM scraped_matches sm
WHERE pi_db.connections.matches.match_id = sm.match_id
  AND sm.original_count > 0 
  AND sm.is_integer_division = true;

UPDATE pi_db.connections.league_seasons ls
SET data_count = matches_summary.matches_with_data,
    updated_at_utc = NOW()
FROM (
    SELECT m.league_id, m.season_id, COUNT(DISTINCT m.match_id) as matches_with_data
    FROM pi_db.connections.matches m
    WHERE m.data_count = 1
    GROUP BY m.league_id, m.season_id
) matches_summary
WHERE ls.league_id = matches_summary.league_id
  AND ls.season_id = matches_summary.season_id;
"#;
