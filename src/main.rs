use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::AttachmentType;
use shuttle_secrets::SecretStore;
use tracing::{error, info};
use rand::Rng;
use std::path::Path;


struct Bot;


#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "!preach" {
            let quotes: [&str; 7] = [
                "Many of us are by the fire forsaken.\n\
                I speak of thine kind, and mine.\n\
                Behold this city! We are kindred, belike two eyes which gaze upon the other.\n\
                Fear not, the dark, my friend.\n\
                And let the feast begin.",
                "One was a wayfaring knight, on an endless, forbidden search.\n\
                Only the Abyss granted closure, if not reunion with his beloved.\n\
                Fear not, the dark, my friend. And let the feast begin",
                "One met the dark with learning. But in the end, learned his knowledge was wanting.\n\
                The world began without knowledge, and without knowledge will it end.\n\
                Dost not this ring clear and true?\n\
                Fear not, the dark, my friend.\n\
                And let the feast begin.",
                "One poor girl slew her own kin, but even so, was embraced, enveloped by the Abyss.\n\
                Twas a comfort that neither moon nor sunless sky afforded her before.\n\
                Fear not, the dark, my friend.\n\
                And let the feast begin.",
                "And so, she lived in fear. Of the dark, of the things that gnawed at her flesh.\n\
                And yet! The Abyss hath yet to produce any such creature!\n\
                Fear not, the dark, my friend.\n\
                And let the feast begin.",
                "And so, despite his weighty armour, he lived in fear. Of a delicate thing, little more than a girl.\n\
                Where fire resideth, shadows twist and shrivel. But in the Abyss, there are shadows none.\n\
                Fear not, the dark, my friend.\n\
                And let the feast begin.",
                "Of all the Fingers, he alone was embraced by the Abyss.\n\
                For he was human, and ne'er a grub.\n\
                Fear not, the dark, my friend.\n\
                And let the feast begin."
                ];
            let n1 = generate_num();
            let fname = format!("/mnt/c/Users/daler/Documents/Rust_Projects/DS_RustBot/audiofiles/VL-0{}.wav",n1+1).to_string();
            if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("").add_file(
                    AttachmentType::Path(Path::new(
                        &fname, 
                    )),
                );
                m
            }).await {
                error!("Error sending message: {:?}", e);
            }
            if let Err(e) = msg.channel_id.say(&ctx.http, &quotes[n1]).await{
                error!("Error sending message: {:?}", e);
            };
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn generate_num() -> usize{
    let mut rng = rand::thread_rng();
    let n1: usize = rng.gen_range(0..7);
    return n1;
}


#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
