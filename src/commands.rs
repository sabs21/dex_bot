use crate::{Context, Error};
use futures::{Stream, StreamExt};
use poise::serenity_prelude as serenity;

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

struct PokemonAutocomplete {
    id: u16,
    name: String,
}
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
struct Effectiveness {
    attacking_type: String,
    value: f32,
}

/// Retrieve information about a Pokemon from the server PokeDex.
///
/// Usage:
/// /dex Blaziken
#[poise::command(prefix_command, slash_command)]
pub async fn dex(
    ctx: Context<'_>,
    #[description = "Retrieve information about a Pokemon from the server PokeDex."]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon: String,
) -> Result<(), Error> {
    // Due to how types are handled for autocomplete value parameters, pokemon id (u16) gets passed in as a String.
    // Hence the need to parse the string for u16.
    let msg = get_pokemon(&pokemon.parse().unwrap()).and_then(|pokemon| 
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
                    if let Some(type2) = pokemon.type2 {
                        if type2 != pokemon.type1.unwrap_or(0) {
                            format!(
                                "{0}, {1}", 
                                &pokemon.type1_name.as_ref().unwrap_or(&"".to_string()), 
                                &pokemon.type2_name.as_ref().unwrap_or(&"".to_string())
                            )
                        } else {
                            format!("{}", &pokemon.type1_name.as_ref().unwrap_or(&"".to_string()))
                        }
                    } else {
                        format!("{}", "None")
                    },
                    true 
                )
                .field(
                    "Abilities",
                    match get_abilities(&pokemon.id) {
                        Ok(rows) => {
                            let mut content: String = "".to_string();
                            for row in rows {
                                content.push_str(&row.name.to_string());
                                content.push_str(":\t");
                                content.push_str(&row.description.to_string());
                                content.push_str("\n");
                            }
                            content
                        },
                        Err(e) => {
                            format!("{}", e.to_string())
                        }
                    },
                    false
                )
                .field(
                    "Egg Groups",
                    match pokemon.egg_group2 {
                        Some(egg_group2) => {
                            if egg_group2 != pokemon.egg_group1.unwrap_or(u16::MAX) {
                                format!(
                                    "{0}, {1}", 
                                    &pokemon.egg_group1_name.as_ref().unwrap_or(&"".to_string()), 
                                    &pokemon.egg_group2_name.as_ref().unwrap_or(&"".to_string())
                                )
                            }
                            else {
                                format!("{}", &pokemon.egg_group1_name.as_ref().unwrap_or(&"".to_string()))
                            }
                        },
                        None => {
                            format!("{}", &pokemon.egg_group1_name.as_ref().unwrap_or(&"".to_string()))
                        }
                    },
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
                .field(
                    "Defensive",
                    match get_effectiveness(&pokemon, Strategy::Defensive) {
                        Ok(rows) => {
                            const LONGEST_NAME_LEN: usize = 13;
                            let mut content: String = "```c\n".to_string();
                            for row in rows {
                                content.push_str(&row.attacking_type);
                                content.push(':');
                                let mut i = LONGEST_NAME_LEN - row.attacking_type.chars().count(); 
                                while i > 0 {
                                    content.push(' ');
                                    i -= 1;
                                }
                                content.push_str(&row.value.to_string());
                                content.push_str("\n");
                            }
                            content.push_str("```");
                            content
                        },
                        Err(e) => format!("```c\nUnable to get defensive type effectiveness\n{0}```", e.to_string())
                    },
                    true 
                )
                .field(
                    "Offensive",
                    match get_effectiveness(&pokemon, Strategy::Offensive) {
                        Ok(rows) => {
                            const LONGEST_NAME_LEN: usize = 13;
                            let mut content: String = "```c\n".to_string();    
                            for row in rows {
                                content.push_str(&row.attacking_type);
                                content.push(':');
                                let mut i = LONGEST_NAME_LEN - row.attacking_type.chars().count();
                                while i > 0 {
                                    content.push(' ');
                                    i -= 1;
                                }
                                content.push_str(&row.value.to_string());
                                content.push_str("\n");
                            }
                            content.push_str("```");
                            content
                        },
                        Err(e) => format!("```c\nUnable to get offensive type effectiveness\n{0}```", e.to_string())
                    },
                    true 
                )
            )
            .components(vec![
                serenity::CreateActionRow::Buttons(vec![
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
            ]))
    );
    ctx.send(msg?).await?;
    Ok(())
}

/// Retrieve necessary autocomplete details for locating a specific Pokemon.
async fn autocomplete_pokemon<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = serenity::AutocompleteChoice> + 'a {
    // Retrieve a list of Pokemon based on the passed in partial text
    let mons: Vec<PokemonAutocomplete> =
        match get_pokemon_autocomplete(partial.to_string()) {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e.to_string());
                vec![]
            }
        };
    futures::stream::iter(mons).map(move |pokemon| {
        serenity::AutocompleteChoice::new(pokemon.name, pokemon.id.to_string())
    })
}
fn get_pokemon_autocomplete(
    name_partial: String,
) -> Result<Vec<PokemonAutocomplete>, Error> {
    let mut mons: Vec<PokemonAutocomplete> = Vec::new();
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare(
        "select [id], [name] from pokemon where [name] like ?1 limit 25",
    )?;
    let mut rows = stmt.query([name_partial + "%"])?;
    while let Some(row) = rows.next()? {
        mons.push(PokemonAutocomplete {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }
    Ok(mons)
}
fn get_pokemon(id: &u16) -> Result<Pokemon, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/queries/get_pokemon.sql") {
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
}
struct Ability {
    name: String,
    description: String,
}
fn get_abilities(pokemon_id: &u16) -> Result<Vec<Ability>, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/queries/get_abilities.sql") {
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

enum Strategy {
    Defensive,
    Offensive,
}
fn get_effectiveness(
    pokemon: &Pokemon,
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
    let mut rows = match stmt.query([pokemon.type1, pokemon.type2]) {
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
