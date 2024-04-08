pub mod eggmoves;
pub mod hmtm;
pub mod tutor;
pub mod levelup;
pub mod type_effectiveness;
pub mod autocomplete;

use poise::serenity_prelude as serenity;
use crate::{Context, Error};

struct Pokemon {
    id: u16,
    pokedex_id: Option<u16>,
    name: String,
    internal_name: Option<String>,
    base_hp: u8,
    base_atk: u8,
    base_def: u8,
    base_spa: u8,
    base_spd: u8,
    base_spe: u8,
    base_total: u16,
    type1: Option<u16>,
    type1_name: Option<String>,
    type2: Option<u16>,
    type2_name: Option<String>,
    egg_group1: Option<u16>,
    egg_group1_name: Option<String>,
    egg_group2: Option<u16>,
    egg_group2_name: Option<String>,
    item1: Option<u16>,
    item1_name: Option<String>,
    item2: Option<u16>,
    item2_name: Option<String>,
    sprite: Option<String>,
}

/// Retrieve information about a Pokemon from the server PokeDex.
///
/// Usage:
/// /dex Blaziken
#[poise::command(prefix_command, slash_command)]
pub async fn dex(
    ctx: Context<'_>,
    #[description = "Retrieve information about a Pokemon from the server PokeDex."]
    #[autocomplete = "autocomplete::autocomplete_pokemon"]
    pokemon: String,
) -> Result<(), Error> {
    // Due to how types are handled for autocomplete value parameters, pokemon id (u16) gets passed in as a String.
    // Hence the need to parse the string for u16.
    let msg = get_pokemon(&pokemon.parse().unwrap()).and_then(|pokemon| { 
        Ok(poise::CreateReply::default()
            .ephemeral(true)
            .embed(serenity::CreateEmbed::new()
                .title(
                    format!(
                        "#{0}: {1}", 
                        pokemon.pokedex_id.unwrap_or(0), 
                        pokemon.name
                    )
                )
                .url(
                    format!(
                        "https://ydarissep.github.io/R.O.W.E-Pokedex/?species={0}&table=speciesTable",
                        pokemon.internal_name.as_ref().unwrap_or(&"".to_string())
                    )
                )
                .colour(get_color_from_type(&pokemon.type1_name.as_ref().unwrap_or(&"".to_string())))
                .thumbnail(pokemon.sprite.as_ref().unwrap_or(&"https://raw.githubusercontent.com/BelialClover/RoweRepo/main/graphics/pokemon/question_mark/circled/front.png".to_string()))
                .field(
                    "Types",
                    &pokemon.type1_name.as_ref().map_or(
                        format!("{}", "None"),
                        |type1_name| {
                            if pokemon.type1.unwrap_or(0) == pokemon.type2.unwrap_or(0) {
                                format!("{}", type1_name)
                            }
                            else {
                                format!(
                                    "{0}, {1}", 
                                    type1_name,
                                    &pokemon.type2_name.as_ref().unwrap_or(&"".to_string())
                                )
                            }
                        }
                    ),
                    true 
                )
                .field(
                    "Abilities",
                    get_abilities(&pokemon.id)
                        .unwrap_or(vec![])
                        .iter()
                        .map(|row| format!(
                            "{0}:\t{1}\n",
                            row.name.to_string(),
                            row.description.to_string()
                        ))
                        .collect::<Vec<String>>()
                        .concat(),
                    false
                )
                .field(
                    "Egg Groups",
                    &pokemon.egg_group1_name.as_ref().map_or(
                        format!("{}", "None"),
                        |egg_group1_name| {
                            if pokemon.egg_group1.unwrap_or(0) == pokemon.egg_group2.unwrap_or(0) {
                                format!("{}", egg_group1_name)
                            }
                            else {
                                format!(
                                    "{0}, {1}", 
                                    egg_group1_name,
                                    &pokemon.egg_group2_name.as_ref().unwrap_or(&"".to_string())
                                )
                            }
                        }
                    ),
                    true 
                )
                .field(
                    "Stats",
                    format!("```c\nHP: \t{0}\nAtk:\t{1}\nDef:\t{2}\nSpA:\t{3}\nSpD:\t{4}\nSpe:\t{5}\nBST:\t{6}```",
                        pokemon.base_hp,
                        pokemon.base_atk,
                        pokemon.base_def,
                        pokemon.base_spa,
                        pokemon.base_spd,
                        pokemon.base_spe,
                        pokemon.base_total
                    ),
                    false
                )
            )
            .components(vec![
                serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new(&("typeeffectiveness_btn__".to_owned() + &pokemon.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("Type Effectiveness"),
                    serenity::CreateButton::new(&("levelup_btn__".to_owned() + &pokemon.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("Level-Up"),
                    serenity::CreateButton::new(&("hmtm_btn__".to_owned() + &pokemon.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("HM/TM"),
                    serenity::CreateButton::new(&("tutor_btn__".to_owned() + &pokemon.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("Tutor"),
                    serenity::CreateButton::new(&("eggmoves_btn__".to_owned() + &pokemon.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("Egg Moves")
                ])
            ])
        )
    });
    ctx.send(msg?).await?;
    Ok(())
}
fn get_pokemon(id: &u16) -> Result<Pokemon, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/dex/queries/get_pokemon.sql") {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e.to_string());
            panic!()
        }
    };
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    stmt.query_row([id], |row| {
        Ok(Pokemon {
            id: row.get(0).unwrap_or(0),
            pokedex_id: row.get(1).unwrap_or(Some(0)),
            name: row.get(2).unwrap_or("".to_string()),
            internal_name: row.get(3).unwrap_or(Some("".to_string())),
            base_hp: row.get(4).unwrap_or(0),
            base_atk: row.get(5).unwrap_or(0),
            base_def: row.get(6).unwrap_or(0),
            base_spa: row.get(7).unwrap_or(0),
            base_spd: row.get(8).unwrap_or(0),
            base_spe: row.get(9).unwrap_or(0),
            base_total: row.get(10).unwrap_or(0),
            type1: row.get(11).unwrap_or(Some(0)),
            type1_name: row.get(12).unwrap_or(Some("".to_string())),
            type2: row.get(13).unwrap_or(Some(0)),
            type2_name: row.get(14).unwrap_or(Some("".to_string())),
            egg_group1: row.get(15).unwrap_or(Some(0)),
            egg_group1_name: row.get(16).unwrap_or(Some("".to_string())),
            egg_group2: row.get(17).unwrap_or(Some(0)),
            egg_group2_name: row.get(18).unwrap_or(Some("".to_string())),
            item1: row.get(19).unwrap_or(Some(0)),
            item1_name: row.get(20).unwrap_or(Some("".to_string())),
            item2: row.get(21).unwrap_or(Some(0)),
            item2_name: row.get(22).unwrap_or(Some("".to_string())),
            sprite: row.get(23).unwrap_or(Some("".to_string())),
        })
    })
    .or_else(|err| Err(err))
}

// Ability field
struct Ability {
    name: String,
    description: String,
}
fn get_abilities(pokemon_id: &u16) -> Result<Vec<Ability>, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/dex/queries/get_abilities.sql") {
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
    let mut abilities: Vec<Ability> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        abilities.push(Ability {
            name: row.get(0)?,
            description: row.get(1)?,
        });
    }
    Ok(abilities)
}

fn get_color_from_type(pokemon_type: &String) -> serenity::model::Color {
    match pokemon_type.as_ref() {
        "Normal" => serenity::model::Color::new(10329457),
        "Fire" => serenity::model::Color::new(15630640),
        "Water" => serenity::model::Color::new(6525168),
        "Electric" => serenity::model::Color::new(13605385),
        "Grass" => serenity::model::Color::new(6463805),
        "Ice" => serenity::model::Color::new(6660510),
        "Fighting" => serenity::model::Color::new(12725800),
        "Poison" => serenity::model::Color::new(10502304),
        "Ground" => serenity::model::Color::new(11638095),
        "Flying" => serenity::model::Color::new(8877751),
        "Psychic" => serenity::model::Color::new(15029629),
        "Bug" => serenity::model::Color::new(8425235),
        "Rock" => serenity::model::Color::new(12099640),
        "Ghost" => serenity::model::Color::new(7559063),
        "Dragon" => serenity::model::Color::new(7288316),
        "Dark" => serenity::model::Color::new(7362374),
        "Steel" => serenity::model::Color::new(9868454),
        "Fairy" => serenity::model::Color::new(14058925),
        _ => serenity::model::Color::LIGHTER_GREY,
    }
}
