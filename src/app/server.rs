use axum::{response::Html, routing::get, Router};

pub async fn run_server() {
    let app = Router::new().route("/", get(home_page));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
