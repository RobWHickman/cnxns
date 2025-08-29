pub const GET_DAILY_PLAYERS: &str = r#"
    SELECT *
    FROM pib_db.connections.daily_selection
    WHERE date = $1
"#;
