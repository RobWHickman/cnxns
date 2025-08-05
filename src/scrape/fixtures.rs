use crate::scrape::errors::ScrapingError;
use crate::scrape::url::{MatchUrl, ScheduleUrl};
use postgres::Client;
use reqwest::blocking;
use scraper::{Html, Selector};

//// Gets a list of fixture URLs for the league/season and writes to the DB.
pub fn scrape_match_urls(schedule_url: ScheduleUrl) -> Result<Vec<MatchUrl>, ScrapingError> {
    let schedule_url_string = schedule_url.to_string();
    let response = blocking::get(&schedule_url_string).map_err(|_| ScrapingError)?;
    let html_content = response.text().map_err(|_| ScrapingError)?;
    let document = Html::parse_document(&html_content);

    let selector: Selector = Selector::parse(schedule_url.match_links_selector()).unwrap();

    let match_urls: Vec<MatchUrl> = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter_map(|href: &str| {
            href.split('/')
                .nth(3)
                .map(|match_id| MatchUrl::new(schedule_url.base_url.clone(), match_id.to_string()))
        })
        .collect();

    Ok(match_urls)
}

pub fn persist_schedule_data(
    client: &mut Client,
    schedule_url: ScheduleUrl,
    match_urls: Vec<MatchUrl>,
) -> Result<(), postgres::Error> {
    let n_matches = match_urls.len() as i32;
    client.execute(
       "INSERT INTO league_seasons (league_id, season_id, league_schedule_url, number_matches, scraped) 
        VALUES ($1, $2, $3, $4, $5)",
       &[
           &(schedule_url.league_id as i32),
           &schedule_url.season_id,
           &schedule_url.to_string(),
           &n_matches,
           &false
       ]
   )?;

    Ok(())
}
