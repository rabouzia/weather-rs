use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::model::gateway::Ready;


struct Handler;

#[async_trait]

impl EventHandler for Handler{
	async fn message(&self, ctx: Context , msg: Message)
	{
		if msg.content == "!ping"
		{

			if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await 
			{
				println!("Error sending. message: {why:?}");
			}
		}
	}
	async fn ready(&self, _: Context, ready: Ready)
	{
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let api_key = std::env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");
    let bot_token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be set");

	let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
	let mut client = Client::builder(&bot_token, intents).event_handler(Handler).await.expect("Err creating bot");
    // println!("{} {}", api_key, bot_token);
	
	if let Err(why) = client.start().await {
		println!("Client Err: {why:?}");
	}


}

