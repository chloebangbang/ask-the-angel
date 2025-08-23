use crate::Data;
use crate::dr_choicers::CHOICERS;

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
// query is a placebo value and won't be used
// I don't know if underscoring it will go through to discord so this is what I do
#[allow(unused_variables)]
pub async fn ask (
    ctx: Context<'_>, 
    #[rest] query: Option<String>
) -> anyhow::Result<()> {
    let max = CHOICERS.len();

    let i = rand::rng().random_range(..max);

    ctx.reply(CHOICERS[i]).await?;
    Ok(())
}