use crate::app::backend::{check_player_connection, get_challenge_players, search_players_by_name};
use crate::app::data_types::{ConnectionRequest, ConnectionResponse};
use crate::app::html::home_page;
use axum::extract::State;
use axum::{
    extract::Query,
    response::Json,
    routing::{get, post},
    Router,
};

use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

pub async fn run_server() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let static_service = ServeDir::new("static");
    let client = Arc::new(client);

    let app = Router::new()
        .route("/", get(challenge_handler))
        .route("/api/search", get(search_handler))
        .route("/api/check-connection", post(connection_handler))
        .nest_service("/static", static_service)
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn challenge_handler(State(client): State<Arc<Client>>) -> axum::response::Html<String> {
    let daily_challenge = get_challenge_players(&client).await.unwrap();
    home_page(daily_challenge).await
}

async fn search_handler(
    Query(params): Query<SearchQuery>,
    State(client): State<Arc<Client>>,
) -> Json<Vec<crate::app::data_types::Player>> {
    let players = search_players_by_name(&client, &params.q)
        .await
        .unwrap_or_else(|_| vec![]);
    Json(players)
}

async fn connection_handler(
    State(client): State<Arc<Client>>,
    Json(payload): Json<ConnectionRequest>,
) -> Json<ConnectionResponse> {
    let new_player_id = payload.new_player_id.clone();
    let last_player_id = payload.player_ids_chain.last().unwrap().clone();
    let connection = check_player_connection(&*client, last_player_id, new_player_id.clone())
        .await
        .unwrap_or(None);

    let response = match connection {
        None => Json(ConnectionResponse {
            success: false,
            message: Some("No shared matches!".to_string()),
            shared_matches: None,
            team_id: None,
            updated_chain: None,
            is_complete: None,
            chain_length: None,
        }),

        Some(player_connection) => {
            let mut updated_chain = payload.player_ids_chain.clone();
            updated_chain.push(new_player_id.clone());
            let chain_length = updated_chain.len();

            let starting_state = get_challenge_players(&*client).await.unwrap();
            let target_player_id = &starting_state.player2.player_id;

            let completion_check = check_player_connection(&*client, new_player_id.clone(), target_player_id.clone())
                .await
                .unwrap_or(None);

            let is_complete = completion_check.is_some();

            Json(ConnectionResponse {
                success: true,
                shared_matches: Some(player_connection.matches_together),
                team_id: Some(player_connection.team_id),
                updated_chain: Some(updated_chain),
                is_complete: Some(is_complete),
                chain_length: Some(chain_length),
                message: None,
            })
        }
    };

    response
}
