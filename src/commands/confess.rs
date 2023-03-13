use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::{ChannelId, GuildId};
use serenity::prelude::{Context, Mentionable};
use serenity::utils::Colour;

use crate::config;

pub async fn run(
    ctx: &Context,
    application_cmd: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    // Check if the command was used in DM
    if application_cmd
        .channel_id
        .to_channel(ctx.http.clone())
        .await
        .unwrap()
        .private()
        .is_none()
    {
        return "This command can only be used in DMs!".to_string();
    }

    let _text_old = options
        .get(0)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let _is_anon_old = options
        .get(1)
        .expect("Expected a bool")
        .resolved
        .as_ref()
        .expect("Expected a bool object");

    let mut _text = String::new();
    let mut _is_anon = true;

    if let CommandDataOptionValue::String(new_text) = _text_old {
        _text = new_text.to_owned();
    } else {
        return "Invalid text!".to_string();
    }

    if let CommandDataOptionValue::Boolean(new_anon) = _is_anon_old {
        _is_anon = *new_anon;
    } else {
        return "Invalid bool!".to_string();
    }

    let mut _title = format!("{} just made a confession!", application_cmd.user.mention());

    if _is_anon {
        _title = "Someone just made an anonymous confession!".to_string();
    }

    let msg = GuildId(config::GUILD_ID)
        .channels(ctx.http.clone())
        .await
        .unwrap()
        .get(ChannelId(config::CONFESSION_CHANNEL_ID).as_ref())
        .unwrap()
        .send_message(ctx.http.clone(), |message| {
            message.embed(|embed| {
                embed
                    .colour(Colour::BLUE)
                    .field("Confession", _text, false)
                    .title(_title)
            })
        })
        .await
        .unwrap();

    format!("Sent confession: {}", msg.link())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("confess")
        .description("Creates a private channel for you to report a bug.")
        .dm_permission(true)
        .create_option(|option| {
            option
                .name("confession")
                .description("Confession you want to make.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("anonymous")
                .description("Choose if you want your confession to be anonymous.")
                .required(true)
                .kind(CommandOptionType::Boolean)
                .add_string_choice("Yes", true)
                .add_string_choice("No", false)
        })
}
