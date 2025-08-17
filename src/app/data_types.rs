use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub player_id: String,
    pub player_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionRequest {
    pub current_chain: Vec<String>,
    pub new_player_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerConnection {
    pub player1: Player,
    pub player2: Player,
    pub matches_together: i32,
    pub team_id: String,
}

#[derive(Debug)]
pub struct GameState {
    pub start_player1: Player,
    pub start_player2: Player,
    pub intermediate_players: Vec<Player>,
    pub connections: Vec<PlayerConnection>,
}
