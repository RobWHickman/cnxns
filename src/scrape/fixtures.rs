use crate::scrape::url::{MatchUrl, ScheduleUrl};
use chrono::NaiveDate;
use postgres::Client;
use reqwest::blocking;
use scraper::{Html, Selector};
use std::thread;
use std::time::Duration;
use toml;

#[derive(Debug)]
pub struct ScrapingError;

//// Gets all league/season combinations from config, checks if fixtures have been found and if not persists fixture URLs to DB.
pub fn get_new_fixtures(mut client: Client, config: &toml::Value) {
    let schedule_urls = get_all_schedule_urls(config);
    for schedule_url in schedule_urls {
        println!("scraping schedule: {:#?}", schedule_url);
        let match_urls = scrape_match_urls(schedule_url.clone()).unwrap();
        thread::sleep(Duration::from_secs(1));

        for match_url in match_urls.clone() {
            println!("scraping match: {:#?}", match_url);
            let match_data = scrape_match_data(match_url).unwrap();
            persist_match_data(&mut client, schedule_url.clone(), match_data).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        let _ = persist_schedule_data(&mut client, schedule_url.clone(), match_urls.clone());
    }
}

#[derive(Debug, Clone)]
struct MatchData {
    match_id: String,
    home_team_id: String,
    away_team_id: String,
    match_date: NaiveDate,
}

fn scrape_match_data(match_url: MatchUrl) -> Result<MatchData, ScrapingError> {
    let match_url_string = match_url.to_string();
    let response = blocking::get(&match_url_string).map_err(|_| ScrapingError)?;
    let html_content = response.text().map_err(|_| ScrapingError)?;
    let document = Html::parse_document(&html_content);

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

    let match_data = MatchData {
        match_id: match_url.match_id,
        home_team_id: team_ids[0].clone(),
        away_team_id: team_ids[1].clone(),
        match_date: match_date,
    };

    Ok(match_data)
}

//// Gets a list of fixture URLs for the league/season and writes to the DB.
fn scrape_match_urls(schedule_url: ScheduleUrl) -> Result<Vec<MatchUrl>, ScrapingError> {
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

fn persist_schedule_data(
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

fn persist_match_data(
    client: &mut Client,
    schedule_url: ScheduleUrl,
    match_data: MatchData,
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

// //// Checks in the DB if the league/season combination has already been scraped.
// fn season_scraping_check(league_id: int, season_id: str) -> bool {
//     !todo
// }

//// Gets a list of ScheduleUrls from the config TOML.
fn get_all_schedule_urls(config: &toml::Value) -> Vec<ScheduleUrl> {
    let fbref_url = config["fbref_ids"]["base_url"].as_str().unwrap();
    let league_configs = config["fbref_ids"]["leagues"].as_array().unwrap();

    let schedule_urls: Vec<ScheduleUrl> = league_configs
        .iter()
        .flat_map(|league| {
            let league_id = league["league_id"].as_integer().unwrap() as u32;
            let league_name = league["league_name"].as_str().unwrap();
            let season_ids = league["season_ids"].as_array().unwrap();

            season_ids.iter().map(move |season| {
                let season_id = season.as_str().unwrap();
                ScheduleUrl::new(
                    fbref_url.to_string(),
                    league_id,
                    league_name.to_string(),
                    season_id.to_string(),
                )
            })
        })
        .collect();

    return schedule_urls;
}
