use crate::serenity::{EventHandler, Interaction};
use crate::Error;
use poise::serenity_prelude as serenity;

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
    pub defensive: f32,
    pub offensive: f32,
}

pub fn get_effectiveness(
    pokemon_id: &u16,
) -> Result<Vec<Effectiveness>, Error> {
    let sql = std::fs::read_to_string("./src/queries/type_effectiveness.sql")
        .unwrap();
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map([pokemon_id], |row| {
        Ok(Effectiveness {
            attacking_type: row.get(0)?,
            defensive: row.get(1)?,
            offensive: row.get(2)?,
        })
    })?;
    Ok(rows
        .into_iter()
        .map(|row| row.unwrap())
        .collect::<Vec<Effectiveness>>())
}

pub struct Handler;
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: serenity::Context, i: Interaction) {
        match i {
            Interaction::Component(i) => {
                match &i.data.custom_id.split_once("__") {
                    Some(("typeeffectiveness_btn", pokemon_id)) => {
                        type_effectiveness_component(
                            ctx,
                            &i,
                            pokemon_id.parse::<u16>().unwrap(),
                        )
                        .await
                        .unwrap()
                    }
                    Some(("levelup_btn", pokemon_id)) => levelup_component(
                        ctx,
                        &i,
                        pokemon_id.parse::<u16>().unwrap(),
                    )
                    .await
                    .unwrap(),
                    Some(("hmtm_btn", pokemon_id)) => hmtm_component(
                        ctx,
                        &i,
                        pokemon_id.parse::<u16>().unwrap(),
                    )
                    .await
                    .unwrap(),
                    Some(("tutor_btn", pokemon_id)) => tutor_component(
                        ctx,
                        &i,
                        pokemon_id.parse::<u16>().unwrap(),
                    )
                    .await
                    .unwrap(),
                    Some(("eggmoves_btn", pokemon_id)) => eggmoves_component(
                        ctx,
                        &i,
                        pokemon_id.parse::<u16>().unwrap(),
                    )
                    .await
                    .unwrap(),
                    Some((&_, _)) => (),
                    None => (),
                };
                ()
            }
            _ => {}
        }
    }
}

pub async fn type_effectiveness_component(
    ctx: serenity::Context,
    i: &serenity::ComponentInteraction,
    pokemon_id: u16,
) -> Result<(), Error> {
    const LONGEST_NAME_LEN: usize = 9;
    let type_effectiveness = get_effectiveness(&pokemon_id);
    let mut defensive: String = "```c\n".to_string();
    let mut offensive: String = defensive.clone();
    type_effectiveness.iter().for_each(|results| {
        results.iter().for_each(|result| {
            let spacing: String = ":".to_owned()
                + &" ".repeat(
                    LONGEST_NAME_LEN - result.attacking_type.chars().count(),
                );
            let defensive_str = result.defensive.to_string();
            let offensive_str = result.offensive.to_string();
            defensive.push_str(
                &(result.attacking_type.clone()
                    + &spacing
                    + &defensive_str
                    + &" ".repeat(defensive_str.chars().count())
                    + "\n"),
            );
            offensive.push_str(
                &(result.attacking_type.clone()
                    + &spacing
                    + &offensive_str
                    + &" ".repeat(offensive_str.chars().count())
                    + "\n"),
            );
        })
    });
    defensive.push_str("```");
    offensive.push_str("```");
    i.create_response(
        ctx,
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .embed(
                    serenity::CreateEmbed::new()
                        .field("Defensive", defensive, true)
                        .field("Offensive", offensive, true),
                )
                .content("Type effectiveness/resistance."),
        ),
    )
    .await?;
    Ok(())
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

pub async fn hmtm_component(
    ctx: serenity::Context,
    i: &serenity::ComponentInteraction,
    _pokemon_id: u16,
) -> Result<(), Error> {
    i.create_response(
        ctx,
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("hmtm not implemented yet."),
        ),
    )
    .await?;
    Ok(())
}

pub async fn tutor_component(
    ctx: serenity::Context,
    i: &serenity::ComponentInteraction,
    _pokemon_id: u16,
) -> Result<(), Error> {
    i.create_response(
        ctx,
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("tutor not implemented yet."),
        ),
    )
    .await?;
    Ok(())
}

pub async fn eggmoves_component(
    ctx: serenity::Context,
    i: &serenity::ComponentInteraction,
    _pokemon_id: u16,
) -> Result<(), Error> {
    i.create_response(
        ctx,
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("eggmoves not implemented yet."),
        ),
    )
    .await?;
    Ok(())
}
