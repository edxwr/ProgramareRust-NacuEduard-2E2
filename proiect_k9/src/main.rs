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

#[derive(sqlx::FromRow)]
struct Episode{
    title : String,
    season : i64,
    episode_num : i64,
    runtime : String
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
        "SELECT url FROM photos WHERE id = ?"
    )
    .bind(nth_doctor)
    .fetch_optional(&ctx.data().database)
    .await?;

    match row
    {
        Some(row) => { ctx.say(row.get::<String, _>("url")).await? }
        None => ctx.say(format!("Nu exista doctorul cu numarul {}!", nth_doctor)).await?
    };

    Ok(())
}

///Episoadele care contin textul dat in titlu
#[poise::command(slash_command)]
async fn episode(ctx: Context<'_>, #[description = "Textul care trebuie cautat"] text : String) -> Result<(), Error>{
    let result : Vec<Episode> = sqlx::query_as(
        "SELECT title, season, episode_num, runtime FROM episodes WHERE title LIKE ?"
    )
    .bind(format!("%{}%", text))
    .fetch_all(&ctx.data().database)
    .await?;

    if result.is_empty(){
        ctx.say(format!("Nu s-a gasit niciun episod cu \"{}\" in titlu!", text)).await?;
    }
    else {
        let mut output : String = String::from("Am gasit urmatoarele episoade:\n");
        for ep in result
        {
            output.push_str(&format!("**S{:02}:E{:02}** | *Titlu* : **{}** | *Durata* : **{}**\n", ep.season, ep.episode_num, ep.title, ep.runtime));
        }
        ctx.say(output).await?;
    }
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
            commands: vec![ping(), quote(), doctor(), episode()],
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