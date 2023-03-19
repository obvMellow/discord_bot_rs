use colored::Colorize;
use serenity::{
    model::{channel::Message, prelude::ChannelId},
    prelude::{Context, Mentionable},
    utils::Colour,
    Error as SerenityError,
};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Logger {
    pub channel_id: ChannelId,
    pub file_name: String,
}

#[derive(Debug)]
pub struct Error {
    pub io: Option<std::io::Error>,
    pub serenity: Option<SerenityError>,
}

impl Logger {
    pub fn new(channel_id: ChannelId, file_name: &str) -> Self {
        Logger {
            channel_id,
            file_name: file_name.to_string(),
        }
    }

    pub async fn message_sent(&self, message: Message, ctx: &Context) -> Result<(), Error> {
        let channel_log = self
            .channel_id
            .send_message(ctx.clone(), |msg| {
                msg.embed(|embed| {
                    embed
                        .colour(Colour::from_rgb(33, 255, 25))
                        .title("Message Sent")
                        .field("Sender", message.author.mention(), true)
                        .field("Content", message.content.clone(), false)
                })
            })
            .await;

        let mut error = Error {
            io: None,
            serenity: None,
        };

        let file_log_msg = format!(
            "{}\n{} {}\n{} {}\n",
            "Message Sent".green().bold(),
            "Author:".green().bold(),
            message.author.tag(),
            "Content:".green().bold(),
            message.content
        );

        println!("{}", file_log_msg);

        match channel_log {
            Ok(_) => (),
            Err(e) => error.serenity = Some(e),
        };

        let file = Path::new(self.file_name.as_str());

        let file = OpenOptions::new().append(true).open(file);

        let file = match file {
            Ok(v) => v,
            Err(e) => {
                error.io = Some(e);
                return Err(error);
            }
        };

        let mut writer = BufWriter::new(file);
        let log = file_log_msg;
        match writeln!(writer, "{}", log) {
            Ok(_) => (),
            Err(e) => {
                error.io = Some(e);
                return Err(error);
            }
        };

        Ok(())
    }
}
