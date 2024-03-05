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

struct PokemonLite {
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

/// Retrieve information about a Pokemon from the server PokeDex.
///
/// Usage:
/// /dex blaziken
#[poise::command(prefix_command, slash_command)]
pub async fn dex(
    ctx: Context<'_>,
    #[description = "Retrieve information about a Pokemon from the server PokeDex."] 
    #[autocomplete = "autocomplete_pokemon"] 
    pokemon: String,
) -> Result<(), Error> {
    println!("Pokemon to find: {0}", pokemon);
    let mon: Pokemon = get_pokemon(&pokemon);
    ctx.say(format!("#{0}: {1}", mon.pokedex_id.unwrap(), mon.name)).await?;
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
fn get_pokemon(name: &String) -> Pokemon {
    let sql = match std::fs::read_to_string("./src/queries/get_pokemon.sql") {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e.to_string());
            panic!()
        }
    };
    let conn = rusqlite::Connection::open("rowedex.db").unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    let mon: Pokemon = match stmt.query_row([name], |row| {
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
    }) {
        Ok(res) => res,
        Err(e) => Pokemon {
            id: 0, 
            pokedex_id: Some(0),
            name: e.to_string(),
            internal_name: Some("".to_string()),
            base_hp: 0,
            base_atk: 0,
            base_def: 0,
            base_spa: 0,
            base_spd: 0,
            base_spe: 0,
            base_total: 0,
            type1: Some(0),
            type1_name: Some("".to_string()),
            type2: Some(0),
            type2_name: Some("".to_string()),
            egg_group1: Some(0),
            egg_group1_name: Some("".to_string()),
            egg_group2: Some(0),
            egg_group2_name: Some("".to_string()),
            item1: Some(0),
            item1_name: Some("".to_string()),
            item2: Some(0),
            item2_name: Some("".to_string()),
            sprite: Some("".to_string())
        }
    };
    mon
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

