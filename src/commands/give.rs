use crate::db::BorsUser;

use colored::Colorize;
use serenity::{
	client::Context,
	framework::standard::{
		macros::command, Args, CommandResult, Delimiter,
	},
	model::{channel::Message, id::UserId},
	utils::Colour,
};
use std::sync::Arc;

#[command]
pub async fn give(ctx: &Context, msg: &Message) -> CommandResult {
	let author_ref = Arc::new(&msg.author);
	info!(
		"Recieved a `give` command from {}!",
		author_ref.clone().name
	);

	let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
	args.advance();

	let guild = msg
		.guild(ctx)
		.await
		.ok_or("Failed to get guild from message!")?;
	let guild_ref = Arc::new(guild);

	let user = args.single::<UserId>()?.to_user(ctx).await?;
	let amount = args.single::<u64>()?;

	let to =
		BorsUser::new(Arc::new(&user), guild_ref.clone(), ctx).await;
	let mut from = BorsUser::new(author_ref, guild_ref, ctx).await;

	let to_ref = Arc::new(to);

	if from.transfer(to_ref.clone(), amount).await.is_ok() {
		msg.channel_id
			.send_message(
				&ctx.http,
				success_embed!(
					"Transfer",
					format!(
						"{} → {}: `{}`",
						from.name,
						to_ref.clone().name,
						amount
					)
				),
			)
			.await?;

		success!(
			"Processed `give` command from {}!",
			msg.author.name.clone()
		);
	} else {
		msg.channel_id
			.send_message(
				&ctx.http,
				fail_embed!(
					"Transfer",
					format!(
						"{} → {}: `{}`\n**FAILED**: Sender does not \
						 have enough coins!",
						from.name,
						to_ref.clone().name,
						amount
					)
				),
			)
			.await?;
		warn!(
			"Recieved an invalid `give` command from {}",
			msg.author.name.clone()
		);
	}
	Ok(())
}
