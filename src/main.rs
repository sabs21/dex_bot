#![warn(clippy::str_to_string)]

mod commands;
mod events;

use poise::serenity_prelude as serenity;
use std::{
    collections::HashMap,
    env::var,
    sync::{Arc, Mutex},
    time::Duration,
};

// Custom user data passed to all command functions
pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
}

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    //env_logger::init();

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::help(), 
            commands::ban(), 
            commands::shutdown(),
            commands::dex(),
            commands::splits()
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot"),
                poise::Prefix::Literal("hey bot,"),
            ],
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                match event {
                    serenity::FullEvent::InteractionCreate { interaction } => {
                        match &interaction.kind() {
                            serenity::InteractionType::Component => {
                                //println!("{}", interaction.as_message_component().unwrap().data.custom_id);
                                match interaction.to_owned().message_component() {
                                    Some(msg_component) => {
                                        match &msg_component.data.custom_id.split_once("__") {
                                            Some(("levelup_btn", pokemon_id)) => {
                                                let content =  match events::get_levelup_sets(&pokemon_id.parse::<u16>().unwrap()) {
                                                    Ok(rows) => {
                                                        let mut output: String = "Level-Up moves\n".to_string();
                                                        for row in rows {
                                                            output.push_str(&row.move_name.to_string());
                                                            output.push_str(" (Level ");
                                                            output.push_str(&row.level.to_string());
                                                            output.push_str(")\n");
                                                        }
                                                        output 
                                                    },
                                                    Err(e) => {
                                                        format!("{}", e.to_string())
                                                    } 
                                                };
                                                msg_component.create_response(
                                                    _ctx,
                                                    serenity::CreateInteractionResponse::Message(
                                                        serenity::CreateInteractionResponseMessage::new()
                                                            .ephemeral(true)
                                                            .content(content)
                                                    )
                                                ).await?
                                            },
                                            Some(("hmtm_btn", _pokemon_id)) => {
                                                msg_component.create_response(
                                                    _ctx,
                                                    serenity::CreateInteractionResponse::Message(
                                                        serenity::CreateInteractionResponseMessage::new()
                                                            .ephemeral(true)
                                                            .content("hmtm not implemented yet.")
                                                    )
                                                ).await?
                                            },
                                            Some(("tutor_btn", _pokemon_id)) => {
                                                msg_component.create_response(
                                                    _ctx,
                                                    serenity::CreateInteractionResponse::Message(
                                                        serenity::CreateInteractionResponseMessage::new()
                                                            .ephemeral(true)
                                                            .content("tutor not implemented yet.")
                                                    )
                                                ).await?
                                            },
                                            Some(("eggmoves_btn", _pokemon_id)) => {
                                                msg_component.create_response(
                                                    _ctx,
                                                    serenity::CreateInteractionResponse::Message(
                                                        serenity::CreateInteractionResponseMessage::new()
                                                            .ephemeral(true)
                                                            .content("eggmoves not implemented yet.")
                                                    )
                                                ).await?
                                            },
                                            Some((&_, _)) => {
                                                msg_component.create_response(
                                                    _ctx, 
                                                    serenity::CreateInteractionResponse::Message(
                                                        serenity::CreateInteractionResponseMessage::new()
                                                            .ephemeral(true)
                                                            .content("Unknown interaction custom id.")
                                                    )
                                                ).await?
                                            },
                                            None => {
                                                msg_component.create_response(
                                                    _ctx, 
                                                    serenity::CreateInteractionResponse::Message(
                                                        serenity::CreateInteractionResponseMessage::new()
                                                            .ephemeral(true)
                                                            .content("Unable to parse interaction custom id.")
                                                    )
                                                ).await?
                                            }
                                        }
                                    },
                                    None => {
                                        println!("Unable to convert interaction into message component.")
                                    }
                                }
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                }
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build();
    
    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}


