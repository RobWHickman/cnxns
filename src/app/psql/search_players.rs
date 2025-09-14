pub const SEARCH_PLAYERS_BY_NAME: &str = r#"
    SELECT DISTINCT player_id, full_name 
    FROM connections.players 
    WHERE LOWER(full_name) LIKE $1 
    LIMIT 10
"#;

pub const GET_PLAYER_CAREER: &str = r#"
with player_matches as (
    select distinct ps.team_id, ps.match_id
    from connections.player_stats ps
    where player_id = 'd2acfbec'
), player_seasons as (
    select 
        m.season_id, 
        t.team_name,
        count(distinct pm.match_id) as match_count
    from player_matches pm
    join connections.matches m
        on m.match_id = pm.match_id
    join connections.teams t
        on t.team_id = pm.team_id
    group by m.season_id, t.team_name
), team_summary as (
    select 
        team_name,
        min(season_id) as start_season,
        max(season_id) as end_season,
        sum(match_count) as total_matches
    from player_seasons
    group by team_name
)
select 
    left(start_season, 4) || '-' || right(end_season, 4) as season_range,
    team_name,
    total_matches
from team_summary
order by start_season, team_name
"#;