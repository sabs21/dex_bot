use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use futures::{Stream, StreamExt};

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

struct Split {
    id: i32,
    name: String,
    internal_name: String
}
/// Retrieve information about a Pokemon from the server PokeDex.
///
/// Usage:
/// /dex blaziken
#[poise::command(prefix_command, slash_command)]
pub async fn splits(
    ctx: Context<'_>,
    #[description = "Retrieve information about a Pokemon from the server PokeDex."] pokemon: String,
) -> Result<(), Error> {
    match get_splits() {
        Ok(res) => {
            for split in res {
                println!(
                    "Id: {0}, Name: {1}, Internal Name: {2}", 
                    split.id,
                    split.name,
                    split.internal_name
                ); 
            }
        },
        Err(e) => {
            println!("{0}", e.to_string())
        }
    };

    ctx.say(format!("Performed query {pokemon}")).await?;
    Ok(())
}

// get_splits is not contained within 'dex' due to an ongoing bug in tokio.
fn get_splits() -> Result<Vec<Split>, Error> {
    let conn = match rusqlite::Connection::open("rowedex.db") {
        Ok(path) => path,
        Err(e) => {
            println!("ERROR (Connection): {0}", e.to_string()); 
            return Err(Box::new(e));
        }
    };
    let mut stmt = match conn.prepare("select * from splits") {
        Ok(prepped) => prepped,
        Err(e) => {
            println!("ERROR (Prepare): {0}", e.to_string()); 
            return Err(Box::new(e));
        }
    };
    let mut rows = match stmt.query([]) {
        Ok(res) => res,
        Err(e) => {
            println!("ERROR (Query): {0}", e.to_string());
            return Err(Box::new(e));
        }
    };
    let mut splits: Vec<Split> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        splits.push(Split {
            id: row.get(0)?,
            name: row.get(1)?,
            internal_name: row.get(2)?
        });
    }
    Ok(splits)
}

struct PokemonLite{
    id: u16,
    name: String,
    //sprite: String
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
    sprite: Option<String>
}
struct Effectiveness {
    attacking_type: String,
    value: f32 
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
    match get_pokemon(&pokemon) {
        Ok(res) => {
            let msg = {
                poise::CreateReply::default()
                    .embed(serenity::CreateEmbed::new()
                        .title(
                            format!(
                                "#{0}: {1}", 
                                res.pokedex_id.unwrap_or(0), 
                                res.name
                            )
                        )
                        .url(
                            format!(
                                "https://ydarissep.github.io/R.O.W.E-Pokedex/?species={0}&table=speciesTable",
                                res.internal_name.as_ref().unwrap_or(&"".to_string())
                            )
                        )
                        .colour(get_color_from_type(&res.type1_name.as_ref().unwrap_or(&"".to_string())))
                        .thumbnail(res.sprite.as_ref().unwrap_or(&"https://raw.githubusercontent.com/BelialClover/RoweRepo/main/graphics/pokemon/question_mark/circled/front.png".to_string()))
                        .field(
                            "Types",
                            match res.type2 {
                                Some(type2) => {
                                    if type2 != res.type1.unwrap_or(u16::MAX) {
                                        format!(
                                            "{0}, {1}", 
                                            &res.type1_name.as_ref().unwrap_or(&"".to_string()), 
                                            &res.type2_name.as_ref().unwrap_or(&"".to_string())
                                        )
                                    }
                                    else {
                                        format!("{}", &res.type1_name.as_ref().unwrap_or(&"".to_string()))
                                    }
                                },
                                None => {
                                    format!("{}", &res.type1_name.as_ref().unwrap_or(&"".to_string()))
                                }
                            },
                            true 
                        )
                        .field(
                            "Egg Groups",
                            match res.egg_group2 {
                                Some(egg_group2) => {
                                    if egg_group2 != res.egg_group1.unwrap_or(u16::MAX) {
                                        format!(
                                            "{0}, {1}", 
                                            &res.egg_group1_name.as_ref().unwrap_or(&"".to_string()), 
                                            &res.egg_group2_name.as_ref().unwrap_or(&"".to_string())
                                        )
                                    }
                                    else {
                                        format!("{}", &res.egg_group1_name.as_ref().unwrap_or(&"".to_string()))
                                    }
                                },
                                None => {
                                    format!("{}", &res.egg_group1_name.as_ref().unwrap_or(&"".to_string()))
                                }
                            },
                            true 
                        )
                        .field(
                            "Stats",
                            format!("```c\nHP: \t{0}\nAtk:\t{1}\nDef:\t{2}\nSpA:\t{3}\nSpD:\t{4}\nSpe:\t{5}\nBST:\t{6}```",
                                res.base_hp,
                                res.base_atk,
                                res.base_def,
                                res.base_spa,
                                res.base_spd,
                                res.base_spe,
                                res.base_total
                            ),
                            false
                        )
                        .field(
                            "Defensive",
                            match get_effectiveness(&res, Strategy::Defensive) {
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
                            match get_effectiveness(&res, Strategy::Offensive) {
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
                            serenity::CreateButton::new("level_up_btn")
                                .style(serenity::ButtonStyle::Secondary)
                                .label("Level-Up"),
                            serenity::CreateButton::new("hmtm_btn")
                                .style(serenity::ButtonStyle::Secondary)
                                .label("HM/TM"),
                            serenity::CreateButton::new("tutor_btn")
                                .style(serenity::ButtonStyle::Secondary)
                                .label("Tutor"),
                            serenity::CreateButton::new("egg_moves_btn")
                                .style(serenity::ButtonStyle::Secondary)
                                .label("Egg Moves")
                        ])
                    ])
            };
            ctx.send(msg).await?
        },
        Err(e) => {
            println!("{0}", e.to_string());
            ctx.say(format!("Pokemon not found.")).await?
        }
    };
    Ok(())
}

/// Retrieve necessary autocomplete details for locating a specific Pokemon.
async fn autocomplete_pokemon<'a>(
    _ctx: Context<'_>,
    partial: &'a str
) -> impl Stream<Item = serenity::AutocompleteChoice> + 'a {
    // Retrieve a list of Pokemon based on the passed in partial text
    let mons: Vec<PokemonLite> = match get_pokemon_lite(partial.to_string()) {
        Ok(res) => res,
        Err(e) => {
            println!("{}", e.to_string());
            vec![]
        }
    };
    futures::stream::iter(mons).map(move |pokemon| {
        serenity::AutocompleteChoice::new(
            format!("{0}", pokemon.name),
            pokemon.name
        )
    })
}
fn get_pokemon_lite(name_partial: String) -> Result<Vec<PokemonLite>, Error> {
    let mut mons: Vec<PokemonLite> = Vec::new();
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare("select [id], [name] from pokemon where [name] like ?1")?;
    let mut rows = stmt.query([name_partial + "%"])?;
    while let Some(row) = rows.next()? {
        mons.push(PokemonLite {
            id: row.get(0)?,
            name: row.get(1)?,
            //sprite: row.get(2)?
        });
    }
    Ok(mons)
}
fn get_pokemon(name: &String) -> Result<Pokemon, rusqlite::Error> {
    let sql = match std::fs::read_to_string("./src/queries/get_pokemon.sql") {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e.to_string());
            panic!()
        }
    };
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    stmt.query_row([name], |row| {
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
            sprite:row.get(23).unwrap_or(Some("".to_string())) 
        })
    })
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
        _ => serenity::model::Color::LIGHTER_GREY
    }
}

enum Strategy {
    Defensive,
    Offensive 
}
fn get_effectiveness(pokemon: &Pokemon, strategy: Strategy) -> Result<Vec<Effectiveness>, Error> {
    let query_path: String;
    match strategy {
        Strategy::Defensive => query_path = "./src/queries/defensive_calculation.sql".to_string(),
        Strategy::Offensive => query_path = "./src/queries/offensive_calculation.sql".to_string()
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
            value: row.get(3)?
        });
    }
    Ok(effects)    
}

/// Ban a user.
///
/// Usage:
/// /ban username
#[poise::command(prefix_command, slash_command)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "User to ban"] user: serenity::User,
    #[description = "Reason for the ban"] reason: String
    ) -> Result<(), Error> {

    ctx.say(format!("Banning {0}. Reason: {1}", user.name, reason)).await?;
    Ok(())
}

/// Exit Hammer+ gracefully
///
/// Requires user confirmation before shutting down.
/// Calling '~shutdown' will not shut the server down instantly.
#[poise::command(prefix_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn shutdown(
    ctx: Context<'_>
) -> Result<(), Error> {
    let shut_down_id = "shut_down";
    let reply = {
        let confirm_button = 
            serenity::CreateButton::new(shut_down_id)
                .style(serenity::ButtonStyle::Danger)
                .label("Shut Down");
        let components = vec![
            serenity::CreateActionRow::Buttons(vec![confirm_button])
        ];

        poise::CreateReply::default()
            .content("Are you sure you want to shut down Hammer+?")
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mut _res) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(20))
        .filter(move |_res| _res.data.custom_id == shut_down_id) 
        .await
    {
        let success_edit = serenity::EditMessage::new()
            .content("Successfully shut down Hammer+")
            .components(vec![]);
        _res.message.edit(ctx, success_edit).await?;
        println!("Shutting down...");
        std::process::exit(0x0);
    }
    Ok(())
}

