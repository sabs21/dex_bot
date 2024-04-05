select 
    t.name as [attacking_type],
    (select name from types tx where bt.type = tx.id) as [type1],
    (select name from types ty where bt2.type = ty.id) as [type2],
    (case when (bt.type = bt2.type) then bt.effectiveness else (bt.effectiveness * bt2.effectiveness) end) as [effectiveness]
from
    base_types bt,
    base_types bt2
inner join
    pokemon p on (p.id = ?1)
inner join
    types t on (
        (bt.base_type = t.id) 
        and (bt2.base_type = t.id) 
    )
where
    (bt.type = p.type1)
    and (bt2.type = p.type2);
