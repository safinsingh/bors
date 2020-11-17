use chrono::prelude::*;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	let time_diff = Utc::now()
		.signed_duration_since(msg.timestamp)
		.num_milliseconds();

	msg.channel_id
		.say(ctx, format!("Pong!\nAPI Latency: {} ms", time_diff))
		.await?;
	Ok(())
}
