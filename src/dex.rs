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
    item1_name: Option<String>,
    item2_name: Option<String>,
    sprite: Option<String>,
}

impl Pokemon {
    fn default() -> Pokemon {
        Pokemon {
            id: 27, // In rowedex.db, the None pokemon has an id of 27
            pokedex_id: Some(0),
            name: "None".to_string(),
            internal_name: Some("SPECIES_NONE".to_string()),
            base_hp: 0,
            base_atk: 0,
            base_def: 0,
            base_spa: 0,
            base_spd: 0,
            base_spe: 0,
            base_total: 0,
            type1: None,
            type1_name: None,
            type2: None,
            type2_name: None,
            egg_group1: None,
            egg_group1_name: None,
            egg_group2: None,
            egg_group2_name: None,
            item1_name: None,
            item2_name: None,
            sprite: Some("https://raw.githubusercontent.com/BelialClover/RoweRepo/main/graphics/pokemon/question_mark/circled/front.png".to_string()) 
        }
    }
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
    let pokemon_id: &Option<u16> = &pokemon.parse::<u16>().map_or_else(|_| None, |id| Some(id));
    let pokemon_result: Result<Pokemon, rusqlite::Error>;
    // If the user did not click an autocomplete option and instead manually typed
    // the name, this if statement will catch that case and try the query again by name.
    if pokemon_id.is_some() {
        pokemon_result = get_pokemon_by_id(&pokemon_id.expect("Pokemon ID incorrectly parsed."));
    }
    else {
        pokemon_result = get_pokemon_by_name(&pokemon);
    }
    let msg = pokemon_result.and_then(|p| { 
        Ok(poise::CreateReply::default()
            .ephemeral(true)
            .embed(serenity::CreateEmbed::new()
                .title(
                    format!(
                        "#{0}: {1}", 
                        p.pokedex_id.unwrap_or(0), 
                        p.name
                    )
                )
                .url(
                    format!(
                        "https://ydarissep.github.io/R.O.W.E-Pokedex/?species={0}&table=speciesTable",
                        p.internal_name.as_ref().unwrap_or(&"".to_string())
                    )
                )
                .colour(get_color_from_type(&p.type1_name.as_ref().unwrap_or(&"".to_string())))
                .thumbnail(p.sprite.unwrap())
                .field(
                    "Types",
                    &p.type1_name.as_ref().map_or(
                        format!("{}", "None"),
                        |type1_name| {
                            if p.type1.unwrap_or(0) == p.type2.unwrap_or(0) {
                                format!("{}", type1_name)
                            }
                            else {
                                format!(
                                    "{0}, {1}", 
                                    type1_name,
                                    &p.type2_name.as_ref().unwrap_or(&"".to_string())
                                )
                            }
                        }
                    ),
                    true 
                )
                .field(
                    "Abilities",
                    get_abilities(&p.id)
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
                    "Held Items",
                    p.item1_name
                        .map_or("".to_owned(), |item| "50% ".to_owned() + &item)
                        .to_string()
                    +
                    &p.item2_name
                        .map_or("".to_owned(), |item| "5% ".to_owned() + &item)
                    ,
                    false
                )
                .field(
                    "Egg Groups",
                    &p.egg_group1_name.as_ref().map_or(
                        format!("{}", "None"),
                        |egg_group1_name| {
                            if p.egg_group1.unwrap_or(0) == p.egg_group2.unwrap_or(0) {
                                format!("{}", egg_group1_name)
                            }
                            else {
                                format!(
                                    "{0}, {1}", 
                                    egg_group1_name,
                                    &p.egg_group2_name.as_ref().unwrap_or(&"".to_string())
                                )
                            }
                        }
                    ),
                    true 
                )
                .field(
                    "Stats",
                    format!("```c\nHP: \t{0}\nAtk:\t{1}\nDef:\t{2}\nSpA:\t{3}\nSpD:\t{4}\nSpe:\t{5}\nBST:\t{6}```",
                        p.base_hp,
                        p.base_atk,
                        p.base_def,
                        p.base_spa,
                        p.base_spd,
                        p.base_spe,
                        p.base_total
                    ),
                    false
                )
            )
            .components(vec![
                serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new(&("typeeffectiveness_btn__".to_owned() + &p.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("Type Effectiveness"),
                    serenity::CreateButton::new(&("levelup_btn__".to_owned() + &p.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("Level-Up"),
                    serenity::CreateButton::new(&("hmtm_btn__".to_owned() + &p.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("HM/TM"),
                    serenity::CreateButton::new(&("tutor_btn__".to_owned() + &p.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("Tutor"),
                    serenity::CreateButton::new(&("eggmoves_btn__".to_owned() + &p.id.to_string()))
                        .style(serenity::ButtonStyle::Secondary)
                        .label("Egg Moves")
                ])
            ])
        )
    });
    ctx.send(msg?).await?;
    Ok(())
}
fn get_pokemon_by_id(id: &u16) -> Result<Pokemon, rusqlite::Error> {
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
        let default_pokemon = Pokemon::default();
        Ok(Pokemon {
            id: row.get(0).unwrap_or(default_pokemon.id),
            pokedex_id: row.get(1).unwrap_or(default_pokemon.pokedex_id),
            name: row.get(2).unwrap_or(default_pokemon.name),
            internal_name: row.get(3).unwrap_or(default_pokemon.internal_name),
            base_hp: row.get(4).unwrap_or(default_pokemon.base_hp),
            base_atk: row.get(5).unwrap_or(default_pokemon.base_atk),
            base_def: row.get(6).unwrap_or(default_pokemon.base_def),
            base_spa: row.get(7).unwrap_or(default_pokemon.base_spa),
            base_spd: row.get(8).unwrap_or(default_pokemon.base_spd),
            base_spe: row.get(9).unwrap_or(default_pokemon.base_spe),
            base_total: row.get(10).unwrap_or(default_pokemon.base_total),
            type1: row.get(11).unwrap_or(default_pokemon.type1),
            type1_name: row.get(12).unwrap_or(default_pokemon.type1_name),
            type2: row.get(13).unwrap_or(default_pokemon.type2),
            type2_name: row.get(14).unwrap_or(default_pokemon.type2_name),
            egg_group1: row.get(15).unwrap_or(default_pokemon.egg_group1),
            egg_group1_name: row.get(16).unwrap_or(default_pokemon.egg_group1_name),
            egg_group2: row.get(17).unwrap_or(default_pokemon.egg_group2),
            egg_group2_name: row.get(18).unwrap_or(default_pokemon.egg_group2_name),
            item1_name: row.get(19).unwrap_or(default_pokemon.item1_name),
            item2_name: row.get(20).unwrap_or(default_pokemon.item2_name),
            sprite: row.get(21).unwrap_or(default_pokemon.sprite),
        })
    })
    .or_else(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => {
            Ok(Pokemon::default())
        },
        _ => Err(err)
    })
}
// Intended as the backup in case the id search fails
fn get_pokemon_by_name(name: &str) -> Result<Pokemon, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/dex/queries/get_pokemon_by_name.sql") {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e.to_string());
            panic!()
        }
    };
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    stmt.query_row([name], |row| {
        let default_pokemon = Pokemon::default();
        Ok(Pokemon {
            id: row.get(0).unwrap_or(default_pokemon.id),
            pokedex_id: row.get(1).unwrap_or(default_pokemon.pokedex_id),
            name: row.get(2).unwrap_or(default_pokemon.name),
            internal_name: row.get(3).unwrap_or(default_pokemon.internal_name),
            base_hp: row.get(4).unwrap_or(default_pokemon.base_hp),
            base_atk: row.get(5).unwrap_or(default_pokemon.base_atk),
            base_def: row.get(6).unwrap_or(default_pokemon.base_def),
            base_spa: row.get(7).unwrap_or(default_pokemon.base_spa),
            base_spd: row.get(8).unwrap_or(default_pokemon.base_spd),
            base_spe: row.get(9).unwrap_or(default_pokemon.base_spe),
            base_total: row.get(10).unwrap_or(default_pokemon.base_total),
            type1: row.get(11).unwrap_or(default_pokemon.type1),
            type1_name: row.get(12).unwrap_or(default_pokemon.type1_name),
            type2: row.get(13).unwrap_or(default_pokemon.type2),
            type2_name: row.get(14).unwrap_or(default_pokemon.type2_name),
            egg_group1: row.get(15).unwrap_or(default_pokemon.egg_group1),
            egg_group1_name: row.get(16).unwrap_or(default_pokemon.egg_group1_name),
            egg_group2: row.get(17).unwrap_or(default_pokemon.egg_group2),
            egg_group2_name: row.get(18).unwrap_or(default_pokemon.egg_group2_name),
            item1_name: row.get(19).unwrap_or(default_pokemon.item1_name),
            item2_name: row.get(20).unwrap_or(default_pokemon.item2_name),
            sprite: row.get(21).unwrap_or(default_pokemon.sprite),
        })
    })
    .or_else(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => {
            let mut default_pokemon = Pokemon::default();
            default_pokemon.name = "Could not find \"".to_string() + name + "\"";
            Ok(default_pokemon)
        },
        _ => Err(err)
    }) 
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
