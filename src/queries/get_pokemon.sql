select 
    p.id,
    p.pokedex_id,
    p.name,
    p.internal_name,
    p.base_hp,
    p.base_atk,
    p.base_def,
    p.base_spa,
    p.base_spd,
    p.base_spe,
    p.base_total,
    p.type1,
    t.name as type1_name,
    p.type2,
    t2.name as type2_name,
    p.egg_group1,
    e.name as egg_group1_name,
    p.egg_group2,
    e2.name as egg_group2_name,
    p.item1,
    i.name as item1_name,
    p.item2,
    i2.name as item2_name,
    p.sprite
from 
    pokemon p
left join
    types t on (t.id = p.type1)
left join
    types t2 on (t2.id = p.type2)
left join
    egg_groups e on (e.id = p.egg_group1)
left join
    egg_groups e2 on (e2.id = p.egg_group2)
left join
    items i on (i.id = p.item1)
left join
    items i2 on (i2.id = p.item2)
where
	p.name like ?1;
