use crate::serenity::{EventHandler, Interaction};
use poise::serenity_prelude as serenity;

use crate::dex::type_effectiveness::type_effectiveness_component;
use crate::dex::levelup::levelup_component;
use crate::dex::hmtm::hmtm_component;
use crate::dex::tutor::tutor_component;
use crate::dex::eggmoves::eggmoves_component;

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
