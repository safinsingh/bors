use crate::{
	db::{get_user_coins, set_user_coins},
	discord::get_user_nick,
};
use anyhow::anyhow;
use serenity::{
	client::Context,
	model::{guild::Guild, user::User},
};

pub(crate) struct BorsUser {
	pub(crate) id: u64,
	pub(crate) name: String,
	pub(crate) balance: u64,
}

impl BorsUser {
	pub(crate) async fn new(
		user: &User,
		guild: &Guild,
		ctx: &Context,
	) -> BorsUser {
		Self {
			id: user.id.as_u64().to_owned(),
			balance: get_user_coins(user),
			name: get_user_nick(guild, ctx, user).await,
		}
	}

	pub(crate) async fn transfer(
		&self,
		to: &BorsUser,
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

	pub(crate) async fn get(&self) -> u64 { self.balance }
}
