use std::env;

use command_types::{Data, Error};
use dotenvy::dotenv;
use google_translator::{translate_one_line, InputLang, OutputLang};
use poise::samples::register_globally;
use poise::serenity_prelude::{ChannelId, Context, GatewayIntents, Member};
use poise::{Event, Framework, FrameworkOptions};

use commands::translate::translate_nick;
use serenity::prelude::Mentionable;

mod command_types;
mod commands;

#[tokio::main]
async fn main() {
    dotenv().ok().unwrap();
    env_logger::init();

    let intents =
        GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILDS | GatewayIntents::GUILD_PRESENCES;

    Framework::builder()
        .options(FrameworkOptions {
            commands: vec![translate_nick()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .token(env::var("BOT_TOKEN").expect("`BOT_TOKEN` env variable is missing"))
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands).await?;
                Ok(command_types::Data {})
            })
        })
        .run()
        .await
        .unwrap();
}

async fn event_handler(
    ctx: &Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::GuildMemberUpdate {
            old_if_available,
            new,
        } => {
            let old_nick = old_if_available
                .clone()
                .unwrap()
                .nick
                .unwrap_or("Nothing".to_string());
            let new_nick = new.nick.clone().unwrap_or("Nothing".to_string());

            if let Ok(_log_channel) = env::var("LOG_CHANNEL") {
                let log_channel = ChannelId(_log_channel.parse::<u64>()?);
                let _ = send_log_message(ctx, old_nick, new_nick, new.clone(), log_channel).await;
            }

        }
        Event::Ready { data_about_bot } => {
            if env::var("LOG_CHANNEL").is_err() {
                log::warn!(
                    "Env variable `LOG_CHANNEL` not set. Nickname changes will not be logged."
                )
            }

            log::info!(
                "Logged in as {} ({})",
                data_about_bot.user.name,
                data_about_bot.user.id
            )
        }
        _ => {}
    }
    Ok(())
}

async fn send_log_message(
    ctx: &Context,
    old_nick: String,
    new_nick: String,
    user: Member,
    channel: ChannelId,
) -> Result<(), Error> {
    let mut response = format!(
        "{}'s nickname has been updated:\n\n`{}` -> `{}`",
        user.mention(),
        old_nick,
        new_nick,
    );

    if new_nick != "Nothing" {
        response.push_str(&format!(
            " (which probably means {})",
            translate_one_line(
                new_nick.to_string(),
                InputLang::Norwegian,
                OutputLang::English
            )
            .await
            .unwrap()
        ))
    }

    if new_nick != old_nick {
        let _ = channel
            .send_message(ctx, |f| {
                f.content(response).allowed_mentions(|am| am.empty_users())
            })
            .await;
    };
    Ok(())
}
