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
pub async fn give(ctx: &Context, msg: &Message) -> CommandResult {
	info!(
		"Recieved a `give` command from {}!",
		msg.author.name.clone()
	);

	let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
	args.advance();

	let guild = msg
		.guild(ctx)
		.await
		.ok_or("Failed to get guild from message!")?;

	let user = args.single::<UserId>()?.to_user(ctx).await?;
	let amount = args.single::<u64>()?;

	let to_nick = guild
		.member(ctx, user.id)
		.await?
		.nick
		.unwrap_or(user.name.clone());
	let to_id = user.id.as_u64().to_string();
	let to_existing = u64::from_be_bytes(
		DB.get(to_id.clone())?
			.unwrap_or(IVec::from(&[0; 8]))
			.to_vec()
			.try_into()
			.unwrap(),
	);

	let from_nick = msg
		.author_nick(ctx)
		.await
		.unwrap_or(msg.author.name.clone());
	let from_id = msg.author.id.as_u64().to_string();
	let from_existing = u64::from_be_bytes(
		DB.get(from_id.clone())?
			.unwrap_or(IVec::from(&[0; 8]))
			.to_vec()
			.try_into()
			.unwrap(),
	);

	if from_existing < amount {
		msg.channel_id
			.send_message(
				&ctx.http,
				fail_embed!(
					"Transfer",
					format!(
						"{} → {}: `{}`\n**FAILED**: Sender does not \
						 have enough coins!",
						from_nick, to_nick, amount
					)
				),
			)
			.await?;
		warn!(
			"Recieved an invalid `give` command from {}",
			msg.author.name.clone()
		);
	} else {
		DB.insert(
			to_id,
			IVec::from(&(to_existing + amount).to_be_bytes()),
		)?;
		DB.insert(
			from_id,
			IVec::from(&(from_existing - amount).to_be_bytes()),
		)?;

		msg.channel_id
			.send_message(
				&ctx.http,
				success_embed!(
					"Transfer",
					format!(
						"{} → {}: `{}`",
						from_nick, to_nick, amount
					)
				),
			)
			.await?;

		success!(
			"Processed `give` command from {}!",
			msg.author.name.clone()
		);
	}
	Ok(())
}
