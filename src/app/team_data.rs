use crate::app::psql::app_schema::{CREATE_TEAMS_TABLE, REFRESH_TEAMS_TABLE, UPDATE_TEAM_COLORS};
use csv::Reader;
use postgres::Client as PgClient;
use std::collections::HashMap;
use std::fs::File;

pub fn refresh_teams_table(db_client: &mut PgClient) -> Result<(), Box<dyn std::error::Error>> {
    db_client.batch_execute(CREATE_TEAMS_TABLE)?;
    db_client.batch_execute(REFRESH_TEAMS_TABLE)?;

    let color_map = parse_team_colors("static/data/team_colours.csv")?;

    for (team_id, (colour1, colour2)) in color_map {
        db_client.execute(UPDATE_TEAM_COLORS,&[&colour1, &colour2, &team_id],
        )?;
    }

    Ok(())
}

fn parse_team_colors(
    file_path: &str,
) -> Result<HashMap<String, (String, String)>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    let mut color_map = HashMap::new();

    for result in reader.records() {
        let record = result?;
        if record.len() >= 4 {
            let team_id = record[0].to_string();
            let colour1 = record[2].to_string();
            let colour2 = record[3].to_string();
            color_map.entry(team_id).or_insert((colour1, colour2));
        }
    }

    Ok(color_map)
}
