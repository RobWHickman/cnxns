use crate::app::backend::{get_challenge_players, search_players_by_name};
use crate::app::html::home_page;
use axum::{extract::Query, response::Json, routing::get, Router};
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
