use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;
use serenity::model::Permissions;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, channel_id: &ChannelId, options: &[CommandDataOption]) -> String {
    let _amount = options
        .get(0)
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

    let messages = channel_id
        .messages(ctx.http.clone(), |builder| builder.limit(amount as u64))
        .await
        .unwrap();

    for message in messages {
        message.delete(ctx.http.clone()).await.unwrap();
    }

    format!("Deleted {} messages", amount).to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("purge")
        .description("Deletes certain amount of messages.")
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
        .create_option(|option| {
            option
                .name("amount")
                .description("Amount of messages to be deleted.")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .dm_permission(false)
}
