pub const GET_DAILY_PLAYERS: &str = r#"
    SELECT *
    FROM pi_db.connections.daily_selection
    WHERE date = $1
"#;
