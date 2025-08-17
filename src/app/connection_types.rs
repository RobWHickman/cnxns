use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
pub struct ConnectionResponse {
    pub success: bool,
    pub shared_matches: Option<i32>,
    pub team_id: Option<String>,
    pub updated_chain: Option<Vec<String>>,
    pub is_complete: Option<bool>,
    pub chain_length: Option<usize>,
}

impl ConnectionResponse {
    pub fn success(
        player_connection: PlayerConnection,
        updated_chain: Vec<String>,
        is_complete: bool,
    ) -> Self {
        let chain_length = updated_chain.len();
        ConnectionResponse {
            success: true,
            shared_matches: Some(player_connection.matches_together),
            team_id: Some(player_connection.team_id),
            updated_chain: Some(updated_chain),
            is_complete: Some(is_complete),
            chain_length: Some(chain_length),
        }
    }

    pub fn failure() -> Self {
        ConnectionResponse {
            success: false,
            shared_matches: None,
            team_id: None,
            updated_chain: None,
            is_complete: None,
            chain_length: None,
        }
    }
}
