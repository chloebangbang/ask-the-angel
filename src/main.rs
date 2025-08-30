mod commands;
mod dr_choicers;

use poise::serenity_prelude as serenity;

pub fn data_dir() -> std::path::PathBuf {
    let mut dir = dirs::data_dir().unwrap();
    dir.push("ata");
    return dir;
}

pub struct Data {
}

impl Data {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[tokio::main]
async fn main() {
    let mut path = data_dir();
    path.push("pwd");
    let token = std::fs::read_to_string(path).unwrap();
    let intents = serenity::GatewayIntents::non_privileged(); // .union(serenity::GatewayIntents::MESSAGE_CONTENT);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            // prefix_options: poise::structs::PrefixFrameworkOptions {
                // prefix: Some(";".into()),
                // additional_prefixes: vec![
                    // poise::Prefix::Literal("; "),
                // ],
                // mention_as_prefix: true,
                // ignore_bots: false,
                // ignore_thread_creation: true,
                // case_insensitive_commands: true,
                // ..Default::default()
            // },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::new())
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    
    client.unwrap().start().await.unwrap();
}
