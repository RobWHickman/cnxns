pub const SEARCH_PLAYERS_BY_NAME: &str = r#"
    SELECT DISTINCT player_id, full_name 
    FROM pib_db.connections.players 
    WHERE LOWER(full_name) LIKE $1 
    LIMIT 10
"#;
