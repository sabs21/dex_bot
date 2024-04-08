use crate::Error;
use poise::serenity_prelude as serenity;

pub struct LevelupMove {
    pub move_name: String,
    pub level: u8,
}

pub fn get_levelup_sets(
    pokemon_id: &u16,
) -> Result<Vec<LevelupMove>, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/dex/queries/get_levelup_set.sql")
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
            move_name: row.get(0)?,
            level: row.get(1)?,
        });
    }
    Ok(set)
}

pub async fn levelup_component(
    ctx: serenity::Context,
    i: &serenity::ComponentInteraction,
    pokemon_id: u16,
) -> Result<(), Error> {
    let content = get_levelup_sets(&pokemon_id).map_or_else(
        |e| format!("{}", e.to_string()),
        |rows| {
            let mut output: String = "Level-Up moves\n".to_string();
            for row in rows {
                output.push_str(&row.move_name.to_string());
                output.push_str(" (Level ");
                output.push_str(&row.level.to_string());
                output.push_str(")\n");
            }
            output
        },
    );
    i.create_response(
        ctx,
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content(content),
        ),
    )
    .await?;
    Ok(())
}
