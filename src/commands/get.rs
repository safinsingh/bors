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
pub async fn get(ctx: &Context, msg: &Message) -> CommandResult {
	info!("Recieved a `get` command from {}!", &msg.author.name);

	let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
	args.advance();

	let guild = msg
		.guild(ctx)
		.await
		.ok_or("Failed to get guild from message!")?;

	let user = args.single::<UserId>()?.to_user(ctx).await?;
	let bors_user = BorsUser::new(&user, &guild, ctx).await;

	let bors_user_balance = bors_user.get().await;

	msg.channel_id
		.send_message(
			&ctx.http,
			success_embed!(
				"Balance",
				format!(
					"{}: `{}`",
					bors_user.name, bors_user_balance
				)
			),
		)
		.await?;

	success!("Processed `get` command from {}!", msg.author.name);
	Ok(())
}
