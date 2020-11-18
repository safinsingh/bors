use crate::db::get_leaderboard;

use colored::Colorize;
use serenity::{
	client::Context,
	framework::standard::{macros::command, CommandResult},
	model::channel::Message,
	utils::Colour,
};

#[command]
pub(crate) async fn all(
	ctx: &Context,
	msg: &Message,
) -> CommandResult {
	info!("Recieved an `all` command from {}!", &msg.author.name);

	let guild = msg
		.guild(ctx)
		.await
		.ok_or("Failed to get guild from message!")?;

	let board = get_leaderboard(&guild, &ctx).await;
	msg.channel_id
		.send_message(&ctx.http, success_embed!(board))
		.await?;

	success!("Processed `all` command from {}!", msg.author.name);
	Ok(())
}
