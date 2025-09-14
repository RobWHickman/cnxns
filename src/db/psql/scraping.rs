pub const UNSCRAPED_MATCHES: &str = r#"
SELECT match_id
FROM connections.matches m 
WHERE NOT EXISTS (
  SELECT * FROM connections.player_stats ps 
  WHERE m.match_id = ps.match_id
)
ORDER BY league_id, season_id    
"#;
