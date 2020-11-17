use crate::{
	db::{get_user_coins, set_user_coins},
	discord::get_user_nick,
};
use anyhow::anyhow;
use serenity::{
	client::Context,
	model::{guild::Guild, user::User},
};

pub struct BorsUser<'a> {
	pub id: u64,
	pub name: String,
	pub balance: u64,
	pub guild: &'a Guild,
	pub ctx: &'a Context,
}

impl<'a> BorsUser<'a> {
	pub async fn new(
		user: &'a User,
		guild: &'a Guild,
		ctx: &'a Context,
	) -> BorsUser<'a> {
		Self {
			id: user.id.as_u64().to_owned(),
			balance: get_user_coins(user),
			name: get_user_nick(guild, ctx, user).await,
			guild: &guild,
			ctx,
		}
	}

	pub async fn transfer(
		&self,
		to: &BorsUser<'a>,
		amount: u64,
	) -> anyhow::Result<()> {
		if self.balance < amount {
			return Err(anyhow!("Not enough coins"));
		} else {
			set_user_coins(to.id, to.balance + amount);
			set_user_coins(self.id, self.balance - amount);
			Ok(())
		}
	}

	pub async fn get(&self) -> u64 { self.balance }
}
