#[derive(Debug, Clone)]
pub struct ScheduleUrl {
    pub base_url: String,
    pub league_id: u32,
    pub league_name: String,
    pub season_id: String,
}

#[derive(Debug, Clone)]
pub struct MatchUrl {
    pub base_url: String,
    pub match_id: String,
}

impl ScheduleUrl {
    pub fn new(base_url: String, league_id: u32, league_name: String, season_id: String) -> Self {
        Self {
            base_url,
            league_id,
            league_name,
            season_id,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}comps/{}/{}/schedule/{}-{}-Scores-and-Fixtures",
            self.base_url, self.league_id, self.season_id, self.season_id, self.league_name
        )
    }

    pub fn match_links_selector(&self) -> &'static str {
        ".left~ .left+ .left a"
    }
}

impl MatchUrl {
    pub fn new(base_url: String, match_id: String) -> Self {
        Self { base_url, match_id }
    }

    pub fn to_string(&self) -> String {
        format!("{}matches/{}", self.base_url, self.match_id)
    }

    pub fn match_data_selector(&self) -> &'static str {
        "strong a"
    }

    pub fn stats_table_selector(&self) -> &'static str {
        "table.stats_table"
    }
}
