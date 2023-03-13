use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{interaction::application_command::ApplicationCommandInteraction, ChannelId},
    prelude::Context,
};

pub async fn run(
    channel_id: &ChannelId,
    ctx: &Context,
    application_cmd: &ApplicationCommandInteraction,
    name_prefix: &str,
    allowed_channel_id: u64,
) -> String {
    crate::commands::bug_report::run(&channel_id, &ctx, &application_cmd, &name_prefix, allowed_channel_id).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("report_member")
        .description("creates a private channel for you to report a member.")
}
