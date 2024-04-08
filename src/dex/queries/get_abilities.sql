select
    a.name,
    a.description
from
    abilities a
left join
    pokemon_ability_relationships pa on (a.id = pa.ability)
where 
    pa.pokemon = ?1
