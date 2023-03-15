use crate::config;
use config::Token;
use openai_gpt_rs::args::ChatArgs;
use openai_gpt_rs::client::Client;
use openai_gpt_rs::response::Content;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;
use std::collections::HashMap;

pub async fn run(
    ctx: &Context,
    application_cmd: &ApplicationCommandInteraction,
    channel_id: &ChannelId,
    options: &[CommandDataOption],
) -> String {
    // Check if the command was used in the correct channel
    if channel_id.as_u64().to_owned() != config::CHAT_COMPLETION_CHANNEL_ID {
        return format!(
            "This command can only be used in: <#{}>",
            config::CHAT_COMPLETION_CHANNEL_ID
        );
    }

    let _client = Client::new(
        config::load("./config.json")
            .expect("No config.json file found!")
            .openai_key()
            .expect("No OpenAI key found in the config.json file!")
            .as_str(),
    );

    let _system_option = options
        .get(0)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let _user_option = options
        .get(1)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let _assistant_option = options
        .get(2)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let mut _system = String::new();
    let mut _user = String::new();
    let mut _assistant = String::new();

    if let CommandDataOptionValue::String(system) = _system_option {
        _system = system.to_string();
    }

    if let CommandDataOptionValue::String(user) = _user_option {
        _user = user.to_string();
    }

    if let CommandDataOptionValue::String(assistant) = _assistant_option {
        _assistant = assistant.to_string();
    }

    let max_tokens = match options.get(1) {
        Some(val) => val.resolved.as_ref().expect("Expected an integer object"),
        None => &CommandDataOptionValue::Integer(2048),
    };

    let mut _max_tokens: u32 = 2048;

    if let CommandDataOptionValue::Integer(new_max_tokens) = max_tokens {
        _max_tokens = *new_max_tokens as u32;
    }

    let mut system: HashMap<String, String> = HashMap::new();
    system.insert("role".to_string(), "system".to_string());
    system.insert("content".to_string(), _system);

    let mut user: HashMap<String, String> = HashMap::new();
    user.insert("role".to_string(), "user".to_string());
    user.insert("content".to_string(), _user);

    let mut assistant: HashMap<String, String> = HashMap::new();
    assistant.insert("role".to_string(), "assistant".to_string());
    assistant.insert("content".to_string(), _assistant);

    let messages: Vec<HashMap<String, String>> = vec![system];

    let args = ChatArgs::new(messages, Some(_max_tokens), Some(1), None, None, None, None);

    application_cmd
        .create_interaction_response(ctx.http.clone(), |message| {
            message.interaction_response_data(|data| data.content("Generating response..."))
        })
        .await
        .unwrap();

    let content = _client
        .create_chat_completion(&args)
        .await
        .unwrap()
        .get_content(0)
        .await
        .unwrap();

    application_cmd
        .create_followup_message(ctx.http.clone(), |message| message.content(content))
        .await
        .unwrap();

    "I guess it went all well".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("chat")
        .description("Completes the given prompt with chat completion AI.")
        .create_option(|option| {
            option
                .name("system")
                .description("Initial message which helps set the behavior of the AI.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("user")
                .description("The message that will be responded to.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("assistant")
                .description("The message that will help lead the AI.")
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
