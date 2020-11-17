use crate::DB;

use colored::Colorize;
use serenity::{
	client::Context,
	framework::standard::{
		macros::command, Args, CommandResult, Delimiter,
	},
	model::{channel::Message, id::UserId},
	utils::Colour,
};
use sled::IVec;
use std::convert::TryInto;

#[command]
pub async fn get(ctx: &Context, msg: &Message) -> CommandResult {
	info!(
		"Recieved a `get` command from {}!",
		msg.author.name.clone()
	);

	let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
	args.advance();

	let guild = msg
		.guild(ctx)
		.await
		.ok_or("Failed to get guild from message!")?;

	let user = args.single::<UserId>()?.to_user(ctx).await?;
	let user_nick = guild
		.member(ctx, user.id)
		.await?
		.nick
		.unwrap_or(user.name.clone());

	let value = u64::from_be_bytes(
		DB.get(user.id.as_u64().to_string())?
			.unwrap_or(IVec::from(&[0; 8]))
			.to_vec()
			.try_into()
			.unwrap(),
	);

	msg.channel_id
		.send_message(
			&ctx.http,
			success_embed!(
				"Balance",
				format!("{}: `{}`", user_nick, value.to_string())
			),
		)
		.await?;

	success!("Processed `get` command from {}!", user_nick);
	Ok(())
}
