use crate::Error;
use poise::serenity_prelude as serenity;

pub struct Effectiveness {
    pub attacking_type: String,
    pub defensive: f32,
    pub offensive: f32,
}

pub fn get_effectiveness(
    pokemon_id: &u16,
) -> Result<Vec<Effectiveness>, Error> {
    let sql = std::fs::read_to_string("./src/dex/queries/type_effectiveness.sql")
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
