mod commands;
mod config;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "test" => commands::test::run(&command.data.options),
                "complete" => {
                    commands::complete::run(&command.channel_id, &command.data.options).await
                }
                "dall_e" => {
                    commands::dall_e::run(
                        &command.channel_id,
                        &ctx,
                        &command,
                        &command.data.options,
                    )
                    .await
                }
                "edit" => commands::edit::run(&command.channel_id, &command.data.options).await,
                "purge" => {
                    commands::purge::run(&ctx, &command.channel_id, &command.data.options).await
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::create_global_application_command(&ctx.http, |command| {
            commands::complete::register(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::dall_e::register(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::edit::register(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::test::register(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::purge::register(command)
        })
        .await
        .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
