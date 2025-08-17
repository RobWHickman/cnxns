use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub player_id: String,
    pub player_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DailyChallenge {
    pub player1: Player,
    pub player2: Player,
    pub shortest_route: i32,
}
