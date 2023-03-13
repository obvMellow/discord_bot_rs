use crate::config;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::ChannelType;
use serenity::model::prelude::{
    ChannelId, PermissionOverwrite, PermissionOverwriteType, RoleId, UserId,
};
use serenity::model::Permissions;
use serenity::prelude::{Context, Mentionable};
use std::time::SystemTime;

pub async fn run(
    channel_id: &ChannelId,
    ctx: &Context,
    application_cmd: &ApplicationCommandInteraction,
    name_prefix: &str,
    allowed_channel_id: u64
) -> String {
    // Check if the command was used in the correct channel
    if channel_id.as_u64().to_owned() != allowed_channel_id{
        return format!(
            "This command can only be used in: <#{}>",
            allowed_channel_id
        );
    }

    let mut rng = StdRng::seed_from_u64(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    );

    let name = format!("{}-{}", name_prefix, rng.gen::<u32>());

    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::ADMINISTRATOR,
            kind: PermissionOverwriteType::Member(UserId(application_cmd.user.id.into())),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::ADMINISTRATOR,
            kind: PermissionOverwriteType::Member(UserId(application_cmd.user.id.into())),
        },
        PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::SPEAK,
            kind: PermissionOverwriteType::Role(RoleId(config::MOD_ROLE_ID)),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::SPEAK,
            kind: PermissionOverwriteType::Role(RoleId(config::MOD_ROLE_ID)),
        },
        PermissionOverwrite {
            allow: Permissions::SPEAK,
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(RoleId(1031279445065089084)),
        },
    ];

    let channel = application_cmd
        .guild_id
        .unwrap()
        .create_channel(ctx.http.clone(), |channel| {
            channel
                .name(name)
                .permissions(permissions)
                .kind(ChannelType::Text)
        })
        .await
        .unwrap();

    format!("Created channel: {}", channel.mention())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("report_bug")
        .description("creates a private channel for you to report a bug.")
}
