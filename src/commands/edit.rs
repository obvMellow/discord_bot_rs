use crate::config;
use openai_gpt_rs::{args::EditArgs, client::Client, response::Content};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;

pub async fn run(channel_id: &ChannelId, options: &[CommandDataOption]) -> String {
    // Check if the command was used in the correct channel
    if channel_id.as_u64().to_owned() != config::EDIT_CHANNEL_ID {
        return format!(
            "This command can only be used in: <#{}>",
            config::EDIT_CHANNEL_ID
        );
    }

    let _prompt_old = options
        .get(0)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let mut _prompt = String::new();

    if let CommandDataOptionValue::String(new_prompt) = _prompt_old {
        _prompt = new_prompt.clone();
    } else {
        return "Invalid prompt!".to_string();
    }

    let _instruct_old = options
        .get(1)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let mut _instruct = String::new();

    if let CommandDataOptionValue::String(new_instruct) = _instruct_old {
        _instruct = new_instruct.clone();
    } else {
        return "Invalid prompt!".to_string();
    }

    let args = EditArgs::new(None, &_instruct, &_prompt, None, None, None);

    let _client = Client::new(
        std::env::var("OPENAI_KEY")
            .expect("Expected a OpenAI Key in the environment")
            .as_str(),
    );

    let content = _client
        .create_edit(&args)
        .await
        .unwrap()
        .get_content(0)
        .await
        .unwrap();

    content
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("edit")
        .description("Edits the given prompt following an instruction.")
        .create_option(|option| {
            option
                .name("prompt")
                .description("The prompt to be edited.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("instruction")
                .description("The instruction.")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
