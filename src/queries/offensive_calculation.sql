select 
    t.name as [attacking_type],
    (select name from types tx where bt.base_type = tx.id) as [type1],
    (select name from types ty where bt2.base_type = ty.id) as [type2],
    (max(bt.effectiveness, bt2.effectiveness)) as [effectiveness]
from
    base_types bt,
    base_types bt2
inner join
    pokemon p on (p.id = ?1)
inner join
    types t on (
        (bt.type = t.id) 
        and (bt2.type = t.id) 
    )
where
    (bt.base_type = p.type1)
    and (bt2.base_type = p.type2);
