use crate::Error;
use poise::serenity_prelude as serenity;

pub struct EggMove {
    pub move_name: String,
}

pub fn get_egg_sets(
    pokemon_id: &u16,
) -> Result<Vec<EggMove>, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/dex/queries/get_egg_set.sql")
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
    let mut set: Vec<EggMove> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        set.push(EggMove {
            move_name: row.get(0)?,
        });
    }
    Ok(set)
}

pub async fn eggmoves_component(
    ctx: serenity::Context,
    i: &serenity::ComponentInteraction,
    pokemon_id: u16,
) -> Result<(), Error> {
    let content = get_egg_sets(&pokemon_id).map_or_else(
        |e| format!("{}", e.to_string()),
        |rows| {
            let mut output: String = "Egg moves\n".to_string();
            for row in rows {
                output.push_str(&row.move_name.to_string());
                output.push_str("\n");
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

