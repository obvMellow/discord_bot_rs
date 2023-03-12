use crate::config;
use crate::openai::response;
use crate::openai::{
    args::{ImageArgs, ImageSize},
    client::Client,
    response::Content,
};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;

pub async fn run(
    channel_id: &ChannelId,
    ctx: &Context,
    application_cmd: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
) -> String {
    // Check if the command was used in the correct channel
    if channel_id.as_u64().to_owned() != config::DALL_E_CHANNEL_ID {
        return format!(
            "This command can only be used in: <#{}>",
            config::COMPLETE_CHANNEL_ID
        );
    }

    let _client = Client::new(
        std::env::var("OPENAI_KEY")
            .expect("Expected a OpenAI Key in the environment")
            .as_str(),
    );

    let _ = application_cmd
        .clone()
        .create_interaction_response(ctx.http.clone(), |response| {
            response.interaction_response_data(|data| data.content("Generating image..."))
        })
        .await
        .unwrap();

    let _prompt = options
        .get(0)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let mut prompt = String::new();

    if let CommandDataOptionValue::String(new_prompt) = _prompt {
        prompt = new_prompt.clone();
    } else {
        return "Invalid prompt!".to_string();
    }

    let _size = options
        .get(1)
        .expect("Expected a size")
        .resolved
        .as_ref()
        .expect("Expected a size object");

    let mut size = String::new();

    if let CommandDataOptionValue::String(new_size) = _size {
        size = new_size.to_string();
    }

    let size_as_enum = ImageSize::Big;

    if size == "256x256" {
        let _size_as_enum = ImageSize::Small;
    } else if size == "512x512" {
        let _size_as_enum = ImageSize::Medium;
    } else if size == "1024x1024" {
        let _size_as_enum = ImageSize::Big;
    }

    let args = ImageArgs::new(prompt.as_str(), None, Some(size_as_enum), None);

    let content = _client
        .create_image(&args)
        .await
        .unwrap()
        .get_content(0)
        .await
        .unwrap();

    let _ = application_cmd
        .create_followup_message(ctx.http.clone(), |response| {
            response.content(format!("<@{}> {}", application_cmd.user.tag(), content))
        })
        .await;

    "".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("dall_e")
        .description("Generates an image with given prompt")
        .create_option(|option| {
            option
                .name("prompt")
                .description("The prompt to be completed.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("size")
                .description("Size of the generated image.")
                .kind(CommandOptionType::String)
                .required(false)
                .add_string_choice("Small", ImageSize::Small)
                .add_string_choice("Medium", ImageSize::Medium)
                .add_string_choice("Big", ImageSize::Big)
        })
}
