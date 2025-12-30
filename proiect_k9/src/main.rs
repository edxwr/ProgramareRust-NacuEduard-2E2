use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use std::env;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error>{
    ctx.say("Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main(){
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("NO DISCORD TOKEN");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data{})
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;

    println!("Starting bot");
    client.unwrap().start().await.unwrap();
}