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
        .nest_service("/static", static_service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn challenge_handler() -> axum::response::Html<String> {
    let challenge_players = get_challenge_players().await.unwrap();
    home_page(challenge_players).await
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
    println!("Connection handler called with: {:?}", payload);
    let players = vec![
        Player {
            player_id: payload.player1_id,
            player_name: "".to_string(),
        },
        Player {
            player_id: payload.player2_id,
            player_name: "".to_string(),
        },
    ];

    let connection = check_player_connection(players).await.unwrap_or(None);
    println!("Connection checked returning: {:?}", connection);
    let response = match connection {
        None => serde_json::json!({"success": false, "message": "No shared matches!"}),
        Some(player_connection) => serde_json::json!({
            "success": true,
            "shared_matches": player_connection.matches_together,
            "team_id": player_connection.team_id
        }),
    };

    println!("Returning response: {:?}", response);
    Json(response)
}
