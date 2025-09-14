pub const GET_DAILY_PLAYERS: &str = r#"
    SELECT *
    FROM connections.daily_selection
    WHERE date = $1
"#;
