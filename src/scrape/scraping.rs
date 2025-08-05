use crate::scrape::fixtures::{persist_schedule_data, scrape_match_urls};
use crate::scrape::matches::{
    persist_basic_match_data, scrape_basic_match_data, scrape_match_document,
};
use crate::scrape::url::ScheduleUrl;
use postgres::Client;
use std::thread;
use std::time::Duration;
use toml;

//// Gets all league/season combinations from config, checks if fixtures have been found and if not persists fixture URLs to DB.
pub fn get_new_fixtures(mut client: Client, config: &toml::Value) {
    let schedule_urls = get_all_schedule_urls(config);
    for schedule_url in schedule_urls {
        println!("scraping schedule: {:#?}", schedule_url);
        let match_urls = scrape_match_urls(schedule_url.clone()).unwrap();
        thread::sleep(Duration::from_secs(1));

        for match_url in match_urls.clone() {
            println!("scraping match: {:#?}", match_url);
            let match_document = scrape_match_document(match_url.clone()).unwrap();
            let match_data = scrape_basic_match_data(match_document, match_url.clone()).unwrap();
            persist_basic_match_data(&mut client, schedule_url.clone(), match_data).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        let _ = persist_schedule_data(&mut client, schedule_url.clone(), match_urls.clone());
    }
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
