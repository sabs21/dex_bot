use poise::serenity_prelude as serenity;
use crate::{Context, Error};
use futures::{Stream, StreamExt};

// Autocomplete pokemon
struct PokemonAutocomplete {
    id: u16,
    name: String,
}
/// Uses end-user's partial input into dex command for autocompleting pokemon
pub async fn autocomplete_pokemon<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = serenity::AutocompleteChoice> + 'a {
    // Retrieve a list of Pokemon based on the passed in partial text
    let mons: Vec<PokemonAutocomplete> =
        get_pokemon_autocomplete(partial.to_string()).unwrap_or_else(|e| {
            println!("{}", e.to_string());
            vec![]
        });
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
