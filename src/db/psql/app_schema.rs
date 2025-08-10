// pub const CREATE_TABLES_SQL: &str = r#"
// CREATE TABLE IF NOT EXISTS daily_selection (
// );

// CREATE TABLE IF NOT EXISTS cached_relationships (
// );

// CREATE TABLE IF NOT EXISTS player_career (
// );

// "#;


// Daily challenges - date, player1_id, player2_id, optimal_distance (daily game pairs)
// Player-teammate relationships - player1_id, player2_id, team_id, season_id, matches_together (cached connections for fast game queries)
// Player career - player_id, team_id, season_id, matches_played, goals_scored, assists, minutes_played (aggregated season stats per team)
