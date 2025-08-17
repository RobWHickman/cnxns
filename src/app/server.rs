use crate::app::backend::{
    check_game_completion, check_player_connection, get_challenge_players, search_players_by_name,
};
use crate::app::connection_types::{ConnectionRequest, ConnectionResponse};
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
) -> Json<Vec<crate::app::entity_types::Player>> {
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
        None => Json(ConnectionResponse::failure()),

        Some(player_connection) => {
            let mut updated_chain = payload.player_ids_chain.clone();
            updated_chain.push(new_player_id.clone());
            let is_complete = check_game_completion(client.as_ref(), &new_player_id)
                .await
                .unwrap_or(false);
            Json(ConnectionResponse::success(
                player_connection,
                updated_chain,
                is_complete,
            ))
        }
    };

    response
}
