pub const CHECK_PLAYERS_CONNECTED: &str = r#"
    SELECT COUNT(*) as shared_matches, shared_team_matches.team_id, t.team_name, t.colour1, t.colour2
    FROM (
        SELECT match_id, team_id
        FROM public.player_stats ps 
        WHERE variable = 'mins_played'
        AND value > 0
        AND player_id IN ($1, $2)
        GROUP BY match_id, team_id
        HAVING COUNT(DISTINCT player_id) = 2
    ) shared_team_matches
    JOIN public.teams t ON t.team_id = shared_team_matches.team_id
    GROUP BY shared_team_matches.team_id, t.team_name, t.colour1, t.colour2
    ORDER BY COUNT(*) DESC
    LIMIT 1;
"#;