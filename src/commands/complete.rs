use crate::config;
use openai_gpt_rs::args::CompletionArgs;
use openai_gpt_rs::client::Client;
use openai_gpt_rs::response::Content;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;

pub async fn run(channel_id: &ChannelId, options: &[CommandDataOption]) -> String {
    // Check if the command was used in the correct channel
    if channel_id.as_u64().to_owned() != config::COMPLETE_CHANNEL_ID {
        return format!("This command can only be used in: <#{}>", config::COMPLETE_CHANNEL_ID)
    }

    let _client = Client::new(
        std::env::var("OPENAI_KEY")
            .expect("Expected a OpenAI Key in the environment")
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

    let args = CompletionArgs::new(_prompt.as_str(), None, None, None, None);

    _client
        .create_completion(&args)
        .await
        .unwrap()
        .get_content(0)
        .await
        .unwrap()
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
        .dm_permission(false)
}
