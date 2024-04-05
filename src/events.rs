use crate::Error;

pub struct LevelupMove {
    //pub pokemon: String,
    pub move_name: String,
    pub level: u8,
}

pub fn get_levelup_sets(
    pokemon_id: &u16,
) -> Result<Vec<LevelupMove>, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/queries/get_levelup_set.sql")
    {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e.to_string());
            panic!()
        }
    };
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    let mut rows = match stmt.query([pokemon_id]) {
        Ok(res) => res,
        Err(e) => {
            println!("ERROR (Query): {0}", e.to_string());
            return Err(e);
        }
    };
    let mut set: Vec<LevelupMove> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        set.push(LevelupMove {
            //pokemon: row.get(0)?,
            move_name: row.get(0)?,
            level: row.get(1)?,
        });
    }
    Ok(set)
}

pub struct Effectiveness {
    pub attacking_type: String,
    pub value: f32,
}
pub enum Strategy {
    Defensive,
    Offensive,
}
pub fn get_effectiveness(
    pokemon_id: &u16,
    strategy: Strategy,
) -> Result<Vec<Effectiveness>, Error> {
    let query_path: String;
    match strategy {
        Strategy::Defensive => {
            query_path = "./src/queries/defensive_calculation.sql".to_string()
        }
        Strategy::Offensive => {
            query_path = "./src/queries/offensive_calculation.sql".to_string()
        }
    }
    let sql = match std::fs::read_to_string(query_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e.to_string());
            panic!()
        }
    };
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    let mut rows = match stmt.query([pokemon_id]) {
        Ok(res) => res,
        Err(e) => {
            println!("ERROR (Query): {0}", e.to_string());
            return Err(Box::new(e));
        }
    };
    let mut effects: Vec<Effectiveness> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        effects.push(Effectiveness {
            attacking_type: row.get(0)?,
            value: row.get(3)?,
        });
    }
    Ok(effects)
}
