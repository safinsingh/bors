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

#[command]
pub(crate) async fn give(
	ctx: &Context,
	msg: &Message,
) -> CommandResult {
	info!("Recieved a `give` command from {}!", &msg.author.name);

	let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
	args.advance();

	let guild = msg
		.guild(ctx)
		.await
		.ok_or("Failed to get guild from message!")?;
	let guild_ref = &guild;

	let user = args.single::<UserId>()?.to_user(ctx).await?;
	let amount = args.single::<u64>()?;

	let to = &BorsUser::new(&user, guild_ref, ctx).await;
	let from = BorsUser::new(&msg.author, guild_ref, ctx).await;

	if from.transfer(to, amount).await.is_ok() {
		msg.channel_id
			.send_message(
				&ctx.http,
				success_embed!(
					"Transfer",
					format!(
						"{} → {}: `{}`",
						from.name, to.name, amount
					)
				),
			)
			.await?;
		success!(
			"Processed `give` command from {}!",
			&msg.author.name
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
						from.name, to.name, amount
					)
				),
			)
			.await?;
		warn!(
			"Recieved an invalid `give` command from {}",
			&msg.author.name
		);
	}
	Ok(())
}
