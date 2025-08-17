use crate::app::backend::{check_player_connection, get_challenge_players, search_players_by_name};
use crate::app::data_types::{ConnectionRequest, Player};
use crate::app::html::home_page;
use axum::{
    extract::Query,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

pub async fn run_server() {
    let static_service = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(challenge_handler))
        .route("/api/search", get(search_handler))
        .route("/api/check-connection", post(connection_handler))
        .route("/api/add-player", post(add_player_handler))
        .nest_service("/static", static_service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn challenge_handler() -> axum::response::Html<String> {
    let game_state = get_challenge_players().await.unwrap();
    println!("Game state: {:?}", game_state);
    home_page(game_state).await
}

async fn search_handler(
    Query(params): Query<SearchQuery>,
) -> Json<Vec<crate::app::data_types::Player>> {
    let players = search_players_by_name(&params.q)
        .await
        .unwrap_or_else(|_| vec![]);
    Json(players)
}

async fn connection_handler(Json(payload): Json<ConnectionRequest>) -> Json<serde_json::Value> {
    println!("Current chain: {:?}", payload.current_chain);
    let new_player_id = payload.new_player_id.clone();
    let last_player_id = payload.current_chain.last().unwrap();
    let players: Vec<Player> = vec![
        Player { player_id: last_player_id.clone(), player_name: "".to_string() },
        Player { player_id: new_player_id.clone(), player_name: "".to_string() },  // Use new_player_id here
    ];
    let connection = check_player_connection(players).await.unwrap_or(None);
    let response = match connection {
        None => serde_json::json!({"success": false, "message": "No shared matches!"}),
        Some(player_connection) => {
            let mut updated_chain = payload.current_chain.clone();
            updated_chain.push(new_player_id.clone());

            let game_state = get_challenge_players().await.unwrap();
            let target_player_id = &game_state.start_player2.player_id;
            
            let completion_check = check_player_connection(vec![
                Player { player_id: new_player_id, player_name: "".to_string() },
                Player { player_id: target_player_id.clone(), player_name: "".to_string() },
            ]).await.unwrap_or(None);
            
            let is_complete = completion_check.is_some();
            println!("IS COMPLETE: {:?}", is_complete);

            serde_json::json!({
                "success": true,
                "shared_matches": player_connection.matches_together,
                "team_id": player_connection.team_id,
                "updated_chain": updated_chain,
                "is_complete": is_complete,
                "chain_length": updated_chain.len()
            })
        }
    };

    println!("Returning response: {:?}", response);
    
    Json(response)
}

async fn add_player_handler(Json(_payload): Json<ConnectionRequest>) -> Json<serde_json::Value> {
    // Add player to game state logic
    Json(serde_json::json!({"success": true}))
}
