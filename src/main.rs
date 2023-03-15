mod commands;
mod config;

use colored::Colorize;
use config::Token;
use openai_gpt_rs::client::Client as AIClient;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Activity;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "test" => commands::test::run(&command.data.options),
                "complete" => {
                    commands::complete::run(
                        &ctx,
                        &command,
                        &command.channel_id,
                        &command.data.options,
                    )
                    .await
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
                "report_bug" => {
                    commands::bug_report::run(
                        &command.channel_id,
                        &ctx,
                        &command,
                        "bug-report",
                        config::BUG_REPORT_CHANNEL_ID,
                    )
                    .await
                }
                "report_member" => {
                    commands::member_report::run(
                        &command.channel_id,
                        &ctx,
                        &command,
                        "member-report",
                        config::MEMBER_REPORT_CHANNEL_ID,
                    )
                    .await
                }
                "confess" => commands::confess::run(&ctx, &command, &command.data.options).await,
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
                eprintln!(
                    "{} cannot respond to slash command: {}",
                    "   Error".red().bold(),
                    why
                );
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "\n{} connected as {}.",
            "   Ready".green().bold(),
            ready.user.name
        );

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

        Command::create_global_application_command(&ctx.http, |command| {
            commands::bug_report::register(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::member_report::register(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::confess::register(command)
        })
        .await
        .unwrap();

        // Set the activity
        let activity = Activity::playing("with your mom");

        ctx.set_presence(Some(activity), OnlineStatus::Online).await;
    }
}

#[tokio::main]
async fn main() {
    let config = config::load("./config.json").expect("No config.json file found!");

    let token = config
        .discord_token()
        .expect("No Discord token found in config.json!");

    let _client = AIClient::new(
        &config
            .openai_key()
            .expect("No OpenAI key found in config.json!"),
    );

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("{} Client error: {:?}", "Error".red().bold(), why);
    }
}
