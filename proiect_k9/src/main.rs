use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use std::env;
use sqlx::SqlitePool;
use sqlx::Row;

struct Data {
    database : SqlitePool
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(sqlx::FromRow)]
struct Quote{
    author : String,
    text : String
}

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error>{
    ctx.say("Pong!").await?;
    Ok(())
}

///Citat random din Doctor Who
#[poise::command(slash_command)]
async fn quote(ctx: Context<'_>) -> Result<(), Error>{
    let result : Quote = sqlx::query_as(
        "SELECT author, text FROM quotes ORDER BY RANDOM() LIMIT 1"
    ).fetch_one(&ctx.data().database).await?;

    ctx.say(format!("> {}\n— *{}*", result.text, result.author)).await?;
    Ok(())
}

///Poza cu al n-lea doctor din Doctor Who
#[poise::command(slash_command)]
async fn doctor(ctx: Context<'_>, #[description = "Al catelea doctor?"] nth_doctor : i64) -> Result<(), Error>{
    let row = sqlx::query(
        &format!("SELECT url FROM photos WHERE id = {}", nth_doctor)
    )
    .fetch_optional(&ctx.data().database)
    .await?;

    println!("{nth_doctor}");

    match row
    {
        Some(row) => { ctx.say(row.get::<String, _>("url")).await? }
        None => ctx.say(format!("Nu exista doctorul cu numarul {}!", nth_doctor)).await?
    };

    Ok(())
}

#[tokio::main]
async fn main(){
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("NO DISCORD TOKEN");
    let database_url = env::var("DATABASE_URL").expect("NO DATABASE URL");
    let db_pool = SqlitePool::connect(&database_url).await.expect("Error when connecting to the database");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), quote(), doctor()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data{ database : db_pool })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;

    println!("Starting bot");
    client.unwrap().start().await.unwrap();
}