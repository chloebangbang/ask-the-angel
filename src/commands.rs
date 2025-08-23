use crate::Data;
use crate::dr_choicers::CHOICERS;

use poise::serenity_prelude as serenity;

use rand::Rng;

type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

pub fn get_commands() -> Vec<poise::Command<Data, anyhow::Error>> {
    vec![ask()]
}

/// Ask The Angel for some advice
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    prefix_command, discard_spare_arguments,
)]
pub async fn ask (
    ctx: Context<'_>, 
    #[rest] query: Option<String>
) -> anyhow::Result<()> {
    let max = CHOICERS.len();

    let i = rand::rng().random_range(..max);
    
    // if the command wasn't invoked with a slash command, don't include an embed
    if !ctx.invocation_string().starts_with('/') {
        ctx.reply(CHOICERS[i]).await?;
    } else {
        // only create an embed if there's query text
        if let Some(query_content) = query {
            // create the embed author
            let mut embed_author = serenity::CreateEmbedAuthor::new(ctx.author().display_name());
            if let Some(avatar) = ctx.author().avatar_url() {
                embed_author = embed_author.icon_url(avatar);
            }

            // create the embed
            let embed = serenity::CreateEmbed::new()
                .author(embed_author)
                .description(query_content);

            let reply = poise::CreateReply::default().content(CHOICERS[i]).embed(embed.clone());
            ctx.send(reply).await?;
        } else {
            ctx.reply(CHOICERS[i]).await?;       
        }
    }
    

    Ok(())
}