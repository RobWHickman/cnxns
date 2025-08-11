pub const CHECK_PLAYERS_CONNECTED: &str = r#"
    SELECT COUNT(*) as shared_matches, team_id
    FROM (
        SELECT match_id, team_id
        FROM connections.public.player_stats ps 
        WHERE variable = 'mins_played'
        AND value > 0
        AND player_id IN ($1, $2)
        GROUP BY match_id, team_id
        HAVING COUNT(DISTINCT player_id) = 2
    ) shared_team_matches
    GROUP BY team_id
    ORDER BY COUNT(*) DESC
    LIMIT 1;
"#;
