pub struct LevelupMove {
    //pub pokemon: String,
    pub move_name: String,
    pub level: u8,
}

pub fn get_levelup_sets(pokemon_id: &u16) -> Result<Vec<LevelupMove>, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/queries/get_levelup_set.sql") {
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
