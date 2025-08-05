use crate::scrape::errors::ScrapingError;
use crate::scrape::url::{MatchUrl, ScheduleUrl};
use chrono::NaiveDate;
use postgres::Client;
use reqwest::blocking;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct BasicMatchData {
    match_id: String,
    home_team_id: String,
    away_team_id: String,
    match_date: NaiveDate,
}

pub fn scrape_match_document(match_url: MatchUrl) -> Result<Html, ScrapingError> {
    let match_url_string = match_url.to_string();
    let response = blocking::get(&match_url_string).map_err(|_| ScrapingError)?;
    let html_content = response.text().map_err(|_| ScrapingError)?;
    let match_document = Html::parse_document(&html_content);
    Ok(match_document)
}

pub fn scrape_basic_match_data(
    document: Html,
    match_url: MatchUrl,
) -> Result<BasicMatchData, ScrapingError> {
    let selector: Selector = Selector::parse(match_url.match_data_selector()).unwrap();

    let team_ids: Vec<String> = document
        .select(&selector)
        .take(2)
        .filter_map(|el| el.value().attr("href"))
        .filter_map(|href| href.split('/').nth(3).map(|id| id.to_string()))
        .collect();

    let match_date: NaiveDate = document
        .select(&selector)
        .nth(2)
        .and_then(|el| {
            let date_str = el.text().collect::<String>();
            NaiveDate::parse_from_str(&date_str, "%A %B %d, %Y").ok()
        })
        .ok_or(ScrapingError)?;

    let match_data = BasicMatchData {
        match_id: match_url.match_id,
        home_team_id: team_ids[0].clone(),
        away_team_id: team_ids[1].clone(),
        match_date: match_date,
    };

    Ok(match_data)
}

pub fn persist_basic_match_data(
    client: &mut Client,
    schedule_url: ScheduleUrl,
    match_data: BasicMatchData,
) -> Result<(), postgres::Error> {
    client.execute(
       "INSERT INTO matches (league_id, season_id, match_id, home_team_id, away_team_id, match_date, data_count) 
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
       &[
           &(schedule_url.league_id as i32),
           &schedule_url.season_id,
           &match_data.match_id,
           &match_data.home_team_id,
           &match_data.away_team_id,
            &match_data.match_date,
           &0i32
       ]
   )?;

    Ok(())
}
