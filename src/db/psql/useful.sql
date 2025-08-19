with scraped_matches as (
select distinct m.match_id, league_id, season_id
from pi_db.connection.matches m 
join pi_db.connection.player_stats ps 
on ps.match_id = m.match_id
)
select ls.league_name, ls.season_id, count(distinct sm.match_id)
from pi_db.connection.league_seasons ls 
join scraped_matches sm
on sm.league_id = ls.league_id and sm.season_id = ls.season_id 
group by ls.league_name, ls.season_id;