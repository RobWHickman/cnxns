use crate::app::entity_types::DailyChallenge;
use axum::response::Html;

pub async fn home_page(daily_challenge: DailyChallenge) -> Html<String> {
    let template =
        std::fs::read_to_string("static/html/page.html").expect("Failed to read page.html");

    let api_prefix = if std::env::var("DEPLOYMENT").unwrap_or_default() == "local" {
        ""
    } else {
        "/cnxns"
    };

    let html = template
        .replace("{{player1_id}}", &daily_challenge.player1.player_id)
        .replace("{{player1_name}}", &daily_challenge.player1.player_name)
        .replace("{{player2_name}}", &daily_challenge.player2.player_name)
        .replace("{{api_prefix}}", api_prefix);

    Html(html)
}
