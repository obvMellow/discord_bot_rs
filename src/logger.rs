use serenity::{
    model::{channel::Message, prelude::ChannelId},
    prelude::{Context, Mentionable},
    utils::Colour,
    Error as SerenityError,
};

#[derive(Debug, Clone)]
pub struct Logger {
    pub channel_id: ChannelId,
    pub file_name: String,
}

#[derive(Debug)]
pub struct Error {
    pub io: Option<std::io::Error>,
    pub serenity_error: Option<SerenityError>,
}

impl Logger {
    pub fn new(channel_id: ChannelId, file_name: &str) -> Self {
        Logger {
            channel_id,
            file_name: file_name.to_string(),
        }
    }

    pub async fn message_sent(&self, message: Message, ctx: &Context) -> Result<(), Error> {
        let log_msg = self
            .channel_id
            .send_message(ctx.clone(), |msg| {
                msg.embed(|embed| {
                    embed
                        .colour(Colour::LIGHT_GREY)
                        .title(format!("{} sent a message", message.author.mention()))
                        .field("Content", message.content, false)
                })
            })
            .await;

        match log_msg {
            Ok(_) => Ok(()),
            Err(e) => Err(Error {
                io: None,
                serenity_error: Some(e),
            }),
        }
    }
}
