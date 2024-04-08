select
    t.name,
    (case when (bt.type = bt2.type)  then bt.effectiveness  else (bt.effectiveness * bt2.effectiveness)  end) as [defensive],
    (max(bt3.effectiveness, bt4.effectiveness)) as [offensive]
from 
    types t
inner join
    pokemon p on (p.id = ?1)
inner join
    base_types bt on ((bt.base_type = t.id)  and (bt.type = p.type1))
inner join
    base_types bt2 on ((bt2.base_type = t.id) and (bt2.type = p.type2))
inner join
    base_types bt3 on ((bt3.base_type = p.type1) and (bt3.type = t.id))
inner join
    base_types bt4 on ((bt4.base_type = p.type2) and (bt4.type = t.id))
order by
    t.name;
