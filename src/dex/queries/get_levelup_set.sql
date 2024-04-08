select 
    --p.name as pokemon,
    m.name as move,
    level
from 
    levelup_learnsets ll
--left join
   -- pokemon p on (p.[id] = ll.[pokemon])
left join
    moves m on (m.[id] = ll.[move])
where
    pokemon = ?1
