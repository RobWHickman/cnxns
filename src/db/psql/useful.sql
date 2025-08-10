with scraped_matches as (
select distinct m.match_id, league_id, season_id 
from connections.public.matches m 
join connections.public.player_stats ps 
on ps.match_id = m.match_id
)
select ls.league_name, ls.season_id, count(distinct sm.match_id)
from connections.public.league_seasons ls 
join scraped_matches sm
on sm.league_id = ls.league_id and sm.season_id = ls.season_id 
group by ls.league_name, ls.season_id;