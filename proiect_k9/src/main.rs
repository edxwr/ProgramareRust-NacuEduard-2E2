use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use ::serenity::futures::StreamExt;
use std::env;
use sqlx::SqlitePool;
use sqlx::Row;
use serenity::all::UserId;

struct Data {
    database : SqlitePool
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/* */
const TIMP_INTRE_INTREBARI : u64 = 5; // secunde
const TIMP_PENTRU_INTREBARE : u64 = 777; // secunde
/* */

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

#[derive(sqlx::FromRow)]
struct User{
    id : String,
    score : i64
}

#[derive(sqlx::FromRow)]
struct Question{
    question : String,
    answer : String
}

#[derive(sqlx::FromRow)]
struct QuestionConfig{
    channel_id : String
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
        ctx.say(format!("**Nu s-a gasit niciun episod cu \"{}\" in titlu!**", text)).await?;
    }
    else {
        let mut output : String = String::from("**🎞️ Am gasit urmatoarele episoade: 🎞️**\n");
        for ep in result
        {
            output.push_str(&format!("**S{:02}**:**E{:02}** | Titlu : **{}** | Durata : **{}**\n", ep.season, ep.episode_num, ep.title, ep.runtime));
        }
        ctx.say(output).await?;
    }
    Ok(())
}

///Clasamentul cu punctele tuturor userilor
#[poise::command(slash_command)]
async fn points(ctx: Context<'_>) -> Result<(), Error>{
    ctx.defer().await?;

    let result : Vec<User> = sqlx::query_as(
        "SELECT id, score FROM users ORDER BY score DESC LIMIT 10"
    )
    .fetch_all(&ctx.data().database)
    .await?;


    if result.is_empty(){
        ctx.say("**Niciun user nu a raspuns inca la vreo intrebare!**").await?;
    }
    else {
        let mut clasament : String = String::from("**🏆 TOP 10 Clasament: 🏆**\n");
        for user in result{
            let id_numar = match user.id.parse::<u64>(){
                Ok(id) => id,
                Err(err) => { println!("**Eroare la parsare user id: {} | {}**\n", user.id, err); continue }
            };

            let user_id = UserId::new(id_numar);
            let username = match user_id.to_user(ctx).await{
                Ok(user_struct) => user_struct.global_name.unwrap_or(user_struct.name),
                Err(err) => { println!("**! Eroare la preluare username pentru user id: {} | {} !**\n", id_numar, err); continue }
            };

            clasament.push_str(&format!("**{}** | {} points\n", username, user.score));
        }
        ctx.say(clasament).await?;
    }
    Ok(())
}

///Seteaza canalul curent pentru intrebarile de trivia
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
async fn set_trivia_channel(ctx : Context<'_>) -> Result<(), Error>{
    let guild_id_struct = ctx.guild_id().ok_or("**Eroare la preluare ID Server. Rulati comanda pe un server!**")?;
    let guild_id = guild_id_struct.to_string();

    let channel_id_struct = ctx.channel_id();
    let channel_id = channel_id_struct.to_string();

    if sqlx::query("INSERT OR REPLACE INTO questions_config (guild_id, channel_id) VALUES (?, ?)")
    .bind(guild_id)
    .bind(channel_id)
    .execute(&ctx.data().database)
    .await
    .is_ok(){
        ctx.say("**Canalul curent a fost selectat pentru a primi intrebari despre Doctor Who cu succes!** ✅").await?;
    }
    else{ 
        ctx.say("**Eroare la setarea canalului curent pentru intrebari!**").await?;
    }

    Ok(())
}

async fn run_questions_loop(db : SqlitePool, ctx : serenity::all::Context, channel_id : serenity::all::ChannelId){
    loop {
        let qna = match sqlx::query_as::<_, Question>(
            "SELECT question, answer FROM questions ORDER BY RANDOM() LIMIT 1"
        )
        .fetch_one(&db)
        .await{
            Ok(qna) => qna,
            Err(e) => {
                println!("Eroare la db la intrebari: {}\n", e);
                if let Err(e) = channel_id.say(&ctx, "**Eroare la acces baza de date pentru intrebari!**").await{
                    println!("Eroare la channel_id.say(): {}", e);
                }
                continue
            }
        };
        let question : String = qna.question;
        let answer : String = qna.answer;

        if let Err(e) = channel_id.say(&ctx, format!("**❓ Intrebare noua! ❓** : **{}**\n*(Aveti {} secunde pentru a raspunde)*", question, TIMP_PENTRU_INTREBARE)).await{
            println!("Eroare la channel_id.say() 185: {}", e);
            break;
        }

        let mut collector = serenity::collector::MessageCollector::new(&ctx.shard)
            .channel_id(channel_id)
            .timeout(tokio::time::Duration::from_secs(TIMP_PENTRU_INTREBARE))
            .stream();

        let mut castigator : Option<String> = None;
        let castigator_id : String;
        while let Some(msg) = collector.next().await{
            if msg.content.to_lowercase().contains(&answer.to_lowercase()){
                castigator = Some(msg.author.global_name.clone().unwrap_or(msg.author.name.clone()));
                castigator_id = msg.author.id.to_string();
                if let Err(e) = sqlx::query(
                    "INSERT INTO users (id, score) VALUES (?, 1) ON CONFLICT(id) DO UPDATE SET score = score + 1"
                )
                .bind(castigator_id)
                .execute(&db)
                .await{
                    println!("Eroare la query bd la intrebari(): {}", e);
                }
                if let Err(e) = msg.reply(&ctx, format!("**🎉 Raspuns corect! ({}) 🎉** Ai primit 1 punct!\nVine urmatoarea intrebare!\nhttps://tenor.com/view/sui-siu-ronaldo-football-portugal-gif-25997537", answer)).await{
                    println!("Eroare la msg.reply() la castigator intrebari: {}", e);
                }
                break;
            }
        }

        if castigator.is_none(){
            if let Err(e) = channel_id.say(&ctx, "**❌ Nimeni nu a raspuns la intrebare in timp. Nu se mai trimit intrebari pana la restart bot! ❌**").await{
                println!("Eroare la channel_id.say() la fara castigator: {}", e);
            }
            break;
        }
        else {
            tokio::time::sleep(tokio::time::Duration::from_secs(TIMP_INTRE_INTREBARI)).await;
        }
    }
}

async fn start_loops(db : SqlitePool, ctx : serenity::all::Context){
    let configs = match sqlx::query_as::<_, QuestionConfig>(
        "SELECT channel_id FROM questions_config"
    )
    .fetch_all(&db)
    .await{
        Ok(vector) => vector,
        Err(e) => {
            println!("Eroare la db la intrebari: {}\n", e);
            Vec::new()
        }
    };

    if configs.is_empty(){
        println!("Niciun canal inregistrat pentru intrebari\n");
    }
    else{
        for config in configs{
            let channel_id_string = config.channel_id;
            let channel_id = match channel_id_string.parse::<u64>(){
                Ok(id) => id,
                Err(e) => { println!("Eroare la parsare channel_id: {}", e); continue; }
            };
            let db_clone = db.clone();
            let ctx_clone = ctx.clone();
            tokio::spawn(async move{
                run_questions_loop(db_clone, ctx_clone, serenity::all::ChannelId::new(channel_id)).await;
            });
        }
    }
}

#[tokio::main]
async fn main(){
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("NO DISCORD TOKEN");
    let database_url = env::var("DATABASE_URL").expect("NO DATABASE URL");
    let db_pool = SqlitePool::connect(&database_url).await.expect("Error when connecting to the database");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![quote(), doctor(), episode(), points(), set_trivia_channel()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let db_clone = db_pool.clone();
                let ctx_clone = ctx.clone();
                tokio::spawn(async move{
                    start_loops(db_clone, ctx_clone).await;
                });

                Ok(Data{ database : db_pool })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;

    println!("Starting bot");
    client.unwrap().start().await.unwrap();
}