use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::{Message, ReactionType};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group,
    },
};

use std::env;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::builder::{CreateEmbed};

#[group]
#[commands(report)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Ready!");
    }


}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("?")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn make_embed<'a>(e: &'a mut CreateEmbed, kind: &str, msg: &Message) -> &'a mut CreateEmbed {
    e.title(kind);
    e.author(|a| {
        a.name(format!("{}#{:04}", msg.author.name, msg.author.discriminator));
        a.icon_url(&msg.author.static_avatar_url().unwrap());
        a
    });
    e.description(&msg.content);
    if !msg.attachments.is_empty() {
        e.field("Attachment", msg.attachments[0].url.to_string(), false);
        e.image(msg.attachments[0].url.to_string());
    }

    e
}

#[command]
async fn report(ctx: &Context, msg: &Message) -> CommandResult {
    if !msg.guild_id.is_none() {
        msg.reply_ping(ctx, "Please send me a direct message to report something").await?;
        return Ok(());
    }

    let react_msg = msg.channel_id.send_message(ctx, |m| {
        m.embed(|e|{
            e.author(|a| {
                a.name("Pokémon Insurgence + ZO");
                a.icon_url("https://images-ext-1.discordapp.net/external/kIw9w5pyk2snlAnBrfRNR9FOw8NDKGp9oKKtmNbgmcY/https/cdn.discordapp.com/icons/83490896879812608/a_22a2e99fdf63a36832215cfe1bdb4fad.jpg");
                a
            });
            e.title("Report Options");
            e.description(":regional_indicator_a: User Hacking
:regional_indicator_b: User Harassment
:regional_indicator_c: Game Suggestion/Bug Report
:regional_indicator_d: Other
❌: Cancel
__**Disclaimer: people using this for non-reporting things will lose access to it**__
");
            e
        })
    }).await?;
    let a = react_msg.react(ctx, ReactionType::Unicode("🇦".to_string()));
    let b = react_msg.react(ctx, ReactionType::Unicode("🇧".to_string()));
    let c = react_msg.react(ctx, ReactionType::Unicode("🇨".to_string()));
    let d = react_msg.react(ctx, ReactionType::Unicode("🇩".to_string()));
    let x = react_msg.react(ctx, ReactionType::Unicode("❌".to_string()));

    a.await?;
    b.await?;
    c.await?;
    d.await?;
    x.await?;

    let kind = if let Some(reaction) = &react_msg.await_reaction(&ctx)
        .author_id(msg.author.id)
        .removed(false)
        .await {
        match reaction.as_inner_ref().emoji.as_data().as_str() {
            "🇦" => "User Harassment",
            "🇧" => "User Harassment",
            "🇨" => "Game Suggestion/Bug Report",
            "🇩" => "Other",
            "❌" => return Ok(()),

            _ => {
                panic!("Unknown reaction");
            }
        }
    } else {
        panic!();
    };
    println!("{}", kind);

    msg.reply_ping(ctx, "Please send a message with your report, include any evidence you might have and are willing to share!
This can be a big help! ").await?;

    if let Some(report_msg) = &msg.channel_id.await_reply(&ctx)
        .await {
        let final_msg = msg.channel_id.send_message(ctx, |m| {
            m.content("Are you sure you want to send this?");
            m.embed(|e| {
                make_embed(e, kind, report_msg);
                e
            });

            m
        }).await?;

        let a = final_msg.react(ctx, ReactionType::Unicode("✅".to_string()));
        let b = final_msg.react(ctx, ReactionType::Unicode("❌".to_string()));

        a.await?;
        b.await?;

        if let Some(reaction) = &final_msg.await_reaction(&ctx)
            .author_id(msg.author.id)
            .removed(false)
            .await {
            match reaction.as_inner_ref().emoji.as_data().as_str() {
                "✅" => {
                    let channel = ChannelId(env::var("MOD_CHANNEL")?.parse::<u64>()?);
                    channel.send_message(ctx, |m| {
                        m.embed(|e|{
                            make_embed(e, kind, report_msg);
                            e
                        })
                    }).await?;
                }
                "❌" => return Ok(()),
                _ => return Ok(())
            }
        }
    }


    Ok(())
}