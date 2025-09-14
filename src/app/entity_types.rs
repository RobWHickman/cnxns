use crate::app::team_data::colors_to_emoji;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub player_id: String,
    pub player_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Team {
    pub team_id: String,
    pub team_name: String,
    pub colour1: Option<String>,
    pub colour2: Option<String>,
    pub color_circles: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DailyChallenge {
    pub player1: Player,
    pub player2: Player,
    pub shortest_route: i32,
}

impl Team {
    pub fn new(
        team_id: String,
        team_name: String,
        colour1: Option<String>,
        colour2: Option<String>,
    ) -> Self {
        let color_circles = colors_to_emoji(&colour1, &colour2);
        Team {
            team_id,
            team_name,
            colour1,
            colour2,
            color_circles,
        }
    }
}
