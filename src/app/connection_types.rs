use serde::{Deserialize, Serialize};
use crate::app::entity_types::Team;

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
    pub team: Team,
}

#[derive(Serialize)]
pub struct ConnectionResponse {
    pub success: bool,
    pub shared_matches: Option<i32>,
    pub team: Option<Team>,
    pub updated_chain: Option<Vec<String>>,
    pub is_complete: Option<bool>,
    pub chain_length: Option<usize>,
    pub message: Option<String>,
    pub final_connection: Option<(i32, Team)>,
}

impl ConnectionResponse {
    pub fn success(
        player_connection: PlayerConnection,
        updated_chain: Vec<String>,
        is_complete: bool,
        final_connection: Option<(i32, Team)>,
    ) -> Self {
        let chain_length = updated_chain.len();
        ConnectionResponse {
            success: true,
            shared_matches: Some(player_connection.matches_together),
            team: Some(player_connection.team),
            updated_chain: Some(updated_chain),
            is_complete: Some(is_complete),
            chain_length: Some(chain_length),
            message: None,
            final_connection
        }
    }

    pub fn failure(message: &str) -> Self {
        ConnectionResponse {
            success: false,
            message: Some(message.to_string()),
            shared_matches: None,
            team: None,
            updated_chain: None,
            is_complete: None,
            chain_length: None,
            final_connection: None
        }
    }
}
