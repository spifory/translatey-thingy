use google_translator::{translate_one_line, InputLang, OutputLang};
use poise::serenity_prelude::Mentionable;
use poise::{command, serenity_prelude::User};

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
    let user_nick = user.nick_in(ctx, ctx.guild_id().unwrap()).await.unwrap();

    let translated_nickname =
        translate_one_line(user_nick, InputLang::Norwegian, OutputLang::English)
            .await
            .unwrap_or(format!(
                "{}'s nickname could not be translated",
                user.mention()
            ));

    let response = format!(
        "{}'s nickname from **Norwegian** to **English** is (_probably_) \"{}\"",
        user.mention(),
        translated_nickname
    );

    ctx.send(|reply| {
        reply
            .content(response)
            .allowed_mentions(|am| am.empty_users())
    })
    .await?;

    Ok(())
}
