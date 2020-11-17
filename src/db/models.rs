use crate::{
	db::{get_user_coins, set_user_coins},
	discord::get_user_nick,
};
use anyhow::anyhow;
use serenity::{
	client::Context,
	model::{guild::Guild, user::User},
};
use std::sync::Arc;

pub struct BorsUser<'a> {
	pub id: u64,
	pub name: String,
	pub balance: u64,
	pub guild: Arc<Guild>,
	pub ctx: Arc<&'a Context>,
}

impl<'a> BorsUser<'a> {
	pub async fn new(
		user: Arc<&'a User>,
		guild: Arc<Guild>,
		ctx: &'a Context,
	) -> BorsUser<'a> {
		Self {
			id: user.id.as_u64().to_owned(),
			balance: get_user_coins(user.clone()),
			name: get_user_nick(guild.clone(), ctx, user.clone())
				.await,
			guild: guild.clone(),
			ctx: Arc::from(ctx),
		}
	}

	pub async fn transfer(
		&mut self,
		to: Arc<BorsUser<'a>>,
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
