use crate::app::backend::{
    check_game_completion, check_player_connection, get_challenge_players, search_players_by_name,
};
use crate::app::connection_types::{ConnectionRequest, ConnectionResponse};
use crate::app::html::home_page;
use axum::extract::State;
use axum::{
    extract::Query,
    http::StatusCode,
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
    let database_url = if env::var("DEPLOYMENT").unwrap_or_default() == "local" {
        env::var("LOCALHOST_DB_STRING").expect("LOCALHOST_DB_STRING must be set")
    } else {
        env::var("PI_DB_STRING").expect("PI_DB_STRING must be set")
    };

    println!("Using database URL: {}", database_url);

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
        .route("/api/remove-player", post(remove_player_handler))
        .nest_service("/static", static_service)
        .with_state(client);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    println!("Server running on http://{}", bind_address);
    axum::serve(listener, app).await.unwrap();
}

async fn challenge_handler(
    State(client): State<Arc<Client>>,
) -> Result<axum::response::Html<String>, StatusCode> {
    let daily_challenge = match get_challenge_players(&client).await {
        Ok(challenge) => challenge,
        Err(e) => {
            println!("Error getting daily challenge: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(home_page(daily_challenge).await)
}
async fn search_handler(
    Query(params): Query<SearchQuery>,
    State(client): State<Arc<Client>>,
) -> Result<Json<Vec<crate::app::entity_types::Player>>, StatusCode> {
    let players = match search_players_by_name(&client, &params.q).await {
        Ok(players) => players,
        Err(e) => {
            println!("Error searching players: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(players))
}

async fn connection_handler(
    State(client): State<Arc<Client>>,
    Json(payload): Json<ConnectionRequest>,
) -> Json<ConnectionResponse> {
    let last_player_id = match payload.player_ids_chain.last() {
        Some(id) => id.clone(),
        None => {
            println!("Error: Empty player chain in connection request");
            return Json(ConnectionResponse::failure("Empty player chain"));
        }
    };
    let new_player_id: String = payload.new_player_id.clone();
    let connection = match check_player_connection(
        &*client,
        last_player_id.clone(),
        new_player_id.clone(),
    )
    .await
    {
        Ok(Some(player_connection)) => player_connection,
        Ok(None) => {
            return Json(ConnectionResponse::failure(
                "Players have never played together",
            ));
        }
        Err(e) => {
            println!(
                "Error: Database error checking connection between {:?} and {:?}: {}",
                last_player_id, new_player_id, e
            );
            return Json(ConnectionResponse::failure(
                "Unable to check player connection",
            ));
        }
    };

    let mut updated_chain = payload.player_ids_chain.clone();
    updated_chain.push(new_player_id.clone());

    let is_complete: bool = match check_game_completion(client.as_ref(), &new_player_id).await {
        Ok(complete) => complete,
        Err(e) => {
            println!(
                "Error checking game completion for {:?}: {}",
                new_player_id, e
            );
            return Json(ConnectionResponse::failure(
                "Unable to check game completion",
            ));
        }
    };

    let final_connection_data = if is_complete {
        let starting_state = match get_challenge_players(&client).await {
            Ok(challenge) => challenge,
            Err(_) => {
                return Json(ConnectionResponse::failure(
                    "Unable to get challenge players",
                ))
            }
        };

        match check_player_connection(
            &client,
            new_player_id.clone(),
            starting_state.player2.player_id,
        )
        .await
        {
            Ok(Some(final_conn)) => Some((final_conn.matches_together, final_conn.team)),
            _ => None,
        }
    } else {
        None
    };

    Json(ConnectionResponse::success(
        connection,
        updated_chain,
        is_complete,
        final_connection_data,
    ))
}

async fn remove_player_handler(
    State(client): State<Arc<Client>>,
    Json(payload): Json<Vec<String>>,
) -> Json<ConnectionResponse> {
    if payload.len() <= 1 {
        return Json(ConnectionResponse::failure("Cannot remove starting player"));
    }

    let mut updated_chain = payload;
    updated_chain.pop();

    let is_complete = if let Some(last_player) = updated_chain.last() {
        check_game_completion(client.as_ref(), last_player)
            .await
            .unwrap_or(false)
    } else {
        false
    };

    Json(ConnectionResponse {
        success: true,
        shared_matches: None,
        team: None,
        updated_chain: Some(updated_chain.clone()),
        is_complete: Some(is_complete),
        chain_length: Some(updated_chain.len()),
        message: None,
        final_connection: None,
    })
}
