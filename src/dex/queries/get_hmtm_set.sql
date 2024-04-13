select 
    m.name as move
from 
    tmhm_learnsets thl
left join
    moves m on (m.[id] = thl.[move])
where
    pokemon = ?1
order by move asc
