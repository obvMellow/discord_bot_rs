use crate::config;
use config::Token;
use openai_gpt_rs::args::CompletionArgs;
use openai_gpt_rs::client::Client;
use openai_gpt_rs::response::Content;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;

pub async fn run(
    ctx: &Context,
    application_cmd: &ApplicationCommandInteraction,
    channel_id: &ChannelId,
    options: &[CommandDataOption],
) -> String {
    // Check if the command was used in the correct channel
    if channel_id.as_u64().to_owned() != config::COMPLETE_CHANNEL_ID {
        return format!(
            "This command can only be used in: <#{}>",
            config::COMPLETE_CHANNEL_ID
        );
    }

    let _client = Client::new(
        config::load("./config.json")
            .expect("No config.json file found!")
            .openai_key()
            .expect("No OpenAI key found in the config.json file!")
            .as_str(),
    );

    let prompt_as_option = options
        .get(0)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let mut _prompt = String::new();

    if let CommandDataOptionValue::String(new_prompt) = prompt_as_option {
        _prompt = new_prompt.clone();
    } else {
        return "Invalid prompt!".to_string();
    }

    let max_tokens = match options.get(1) {
        Some(val) => val.resolved.as_ref().expect("Expected an integer object"),
        None => &CommandDataOptionValue::Integer(16),
    };

    let mut _max_tokens: u32 = 16;

    if let CommandDataOptionValue::Integer(new_max_tokens) = max_tokens {
        _max_tokens = *new_max_tokens as u32;
    }

    let args = CompletionArgs::new(_prompt.as_str(), Some(_max_tokens), None, None, None);

    application_cmd
        .create_interaction_response(ctx.http.clone(), |resp| {
            resp.interaction_response_data(|data| data.content("Generating response..."))
        })
        .await
        .unwrap();

    let content = _client
        .create_completion(&args)
        .await
        .unwrap()
        .get_content(0)
        .await
        .unwrap();

    application_cmd
        .create_followup_message(ctx.http.clone(), |msg| msg.content(content))
        .await
        .unwrap();

    "I guess it worked".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("complete")
        .description("Completes the given prompt")
        .create_option(|option| {
            option
                .name("prompt")
                .description("The prompt to be completed")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("max_tokens")
                .description("Max amount of tokens (one token is about a word or punctuation) the completion can contain.")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(4096)
                .required(false)
        })
        .dm_permission(false)
}
