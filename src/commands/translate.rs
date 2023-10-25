use google_translator::{translate_one_line, InputLang, OutputLang};
use poise::{command, serenity_prelude::User};
use serenity::prelude::Mentionable;

use crate::command_types;

/// Translate one of Epic's Norwegian nicknames
#[command(
    guild_only,
    slash_command,
    rename = "translate-nick",
    context_menu_command = "Translate Norwegian Nickname"
)]
pub async fn translate_nick(
    ctx: command_types::Context<'_>,
    #[description = "The user who's nickname to translate."] user: User,
) -> Result<(), command_types::Error> {
    let user_nick = user.nick_in(ctx, ctx.guild_id().unwrap()).await;

    if user_nick.is_none() {
        let content = format!("{} has no nickname to be translated", user.mention());
        let _ = ctx
            .send(|b| b.content(content).allowed_mentions(|am| am.empty_users()))
            .await;
        Ok(())
    } else {
        match translate_one_line(user_nick.unwrap(), InputLang::Norwegian, OutputLang::English).await {
            Err(err) => {
                let content = format!("{}'s nickname could not be translated: {}", user.mention(), err);
                log::error!("{}", content);
                let _ = ctx.send(|b| b.content(content).allowed_mentions(|am| am.empty_users())).await;
                Ok(())
            },
            Ok(translated_nick) => {
                let content = format!("{}'s nickname from **Norwegian** to **English** is _probably_ \"{}\"", user.mention(), translated_nick);
                let _ = ctx.send(|b| b.content(content).allowed_mentions(|am| am.empty_users())).await;
                Ok(())
            }
        }
    }
}
