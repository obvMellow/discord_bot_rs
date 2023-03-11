use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let msg = options
        .get(0)
        .expect("Expected a string")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    if let CommandDataOptionValue::String(msg) = msg {
        msg.to_string()
    } else {
        "Error".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("test")
        .description("A test command")
        .create_option(|option| {
            option
                .name("message")
                .description("Message to respond with.")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
