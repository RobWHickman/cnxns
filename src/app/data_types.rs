use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub player_id: String,
    pub player_name: String,
}

pub struct PlayerConnection {
    pub player1: Player,
    pub player2: Player,
    pub matches_together: i32,
    pub team_id: String,
}

pub struct GameState {
    pub start_player1: Player,
    pub start_player2: Player,
    pub intermediate_players: Vec<Player>,
    pub connections: Vec<PlayerConnection>,
}

// impl GameState {
//     pub fn add_intermediate_player(&mut self, player: Player) -> Result<(), String> {
//         let last_player = self.get_last_player();
//         let players_connection: PlayerConnection = get_matches_together(&last_player.player_id, &player.player_id)?;

//         if players_connection.matches_together == 0 {
//             return Err("Players never played together".to_string());
//         }

//         self.connections.push(PlayerConnection {
//             player1: last_player.clone(),
//             player2: player.clone(),
//             matches_together,
//         });

//         self.intermediate_players.push(player);
//         Ok(())
//     }

//     fn get_last_player(&self) -> &Player {
//         self.intermediate_players.last().unwrap_or(&self.start_player1)
//     }
// }
