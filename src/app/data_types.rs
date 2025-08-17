use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub player_id: String,
    pub player_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionRequest {
    pub player_ids_chain: Vec<String>,
    pub new_player_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerConnection {
    pub player1_id: String,
    pub player2_id: String,
    pub matches_together: i32,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DailyChallenge {
    pub player1: Player,
    pub player2: Player,
    pub shortest_route: i32,
}

#[derive(Serialize)]
pub struct ConnectionResponse {
    pub success: bool,
    pub shared_matches: Option<i32>,
    pub team_id: Option<String>,
    pub updated_chain: Option<Vec<String>>,
    pub is_complete: Option<bool>,
    pub chain_length: Option<usize>,
    pub message: Option<String>,
}
