select 
    m.name as move,
    level
from 
    levelup_learnsets ll
left join
    moves m on (m.[id] = ll.[move])
where
    pokemon = ?1
order by level asc
