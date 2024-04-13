select 
    m.name as move
from 
    tutor_learnsets tl
left join
    moves m on (m.[id] = tl.[move])
where
    pokemon = ?1
order by move asc
