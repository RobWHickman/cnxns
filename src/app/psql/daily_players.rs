pub const GET_DAILY_PLAYERS: &str = r#"
    SELECT *
    FROM public.daily_selection
    WHERE date = $1
"#;
