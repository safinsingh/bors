use chrono::prelude::*;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
};

#[command]
pub(crate) async fn ping(
	ctx: &Context,
	msg: &Message,
) -> CommandResult {
	msg.channel_id
		.say(
			ctx,
			format!(
				"Pong!\nAPI Latency: {} ms",
				Utc::now()
					.signed_duration_since(msg.timestamp)
					.num_milliseconds()
			),
		)
		.await?;
	Ok(())
}
