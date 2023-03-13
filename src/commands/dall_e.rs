use crate::config;
use openai_gpt_rs::{
    args::{ImageArgs, ImageSize},
    client::Client,
    response::Content,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;
use std::time::SystemTime;

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
            config::DALL_E_CHANNEL_ID
        );
    }

    let _client = Client::new(
        std::env::var("OPENAI_KEY")
            .expect("Expected a OpenAI Key in the environment")
            .as_str(),
    );

    application_cmd
        .clone()
        .create_interaction_response(ctx.http.clone(), |response| {
            response.interaction_response_data(|data| data.content("Generating image..."))
        })
        .await
        .unwrap();

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

    let _amount = options
        .get(1)
        .expect("Expected an int")
        .resolved
        .as_ref()
        .expect("Expected an int object");

    let amount: usize;

    if let CommandDataOptionValue::Integer(new_amount) = _amount {
        amount = new_amount.to_owned() as usize;
    } else {
        amount = 1;
    }

    let _size = options
        .get(2)
        .expect("Expected a size")
        .resolved
        .as_ref()
        .expect("Expected a size object");

    let mut size = String::new();

    if let CommandDataOptionValue::String(new_size) = _size {
        size = new_size.to_string();
    }

    let size_as_enum = match size.as_str() {
        "256x256" => ImageSize::Small,
        "512x512" => ImageSize::Medium,
        "1024x1024" => ImageSize::Big,
        _ => ImageSize::Big,
    };

    let args = ImageArgs::new(_prompt.as_str(), Some(amount), Some(size_as_enum), None);

    let json = _client
        .create_image(&args)
        .await
        .unwrap()
        .get_json()
        .await
        .unwrap();

    let mut contents: Vec<String> = Vec::new();

    for i in 0..amount {
        contents.insert(
            contents.len(),
            json.as_object()
                .unwrap()
                .get("data")
                .unwrap()
                .as_array()
                .unwrap()
                .get(i)
                .unwrap()
                .as_object()
                .unwrap()
                .get("url")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        )
    }

    let mut rng = StdRng::seed_from_u64(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    );

    let random_number = rng.gen::<u32>();

    let thread = application_cmd
        .channel_id
        .create_public_thread(
            ctx.http.clone(),
            application_cmd
                .channel_id
                .messages(ctx.http.clone(), |builder| builder.limit(2))
                .await
                .unwrap()
                .first()
                .unwrap()
                .id,
            |f| {
                f.auto_archive_duration(1440)
                    .name(format!("img-{}", random_number))
            },
        )
        .await
        .unwrap();

    for content in contents {
        thread
            .send_message(ctx.http.clone(), |f| f.content(content))
            .await
            .unwrap();
    }

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
                .name("amount")
                .description("Amount of images to be generated.")
                .kind(CommandOptionType::Integer)
                .required(true)
                .min_int_value(1)
                .max_int_value(10)
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
        .dm_permission(false)
}
