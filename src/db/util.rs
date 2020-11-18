use crate::{discord::get_user_nick, DB};

use futures::stream::{self, StreamExt};
use lazysort::SortedBy;
use serenity::{
	http::CacheHttp,
	model::{guild::Guild, id::UserId, user::User},
};
use sled::IVec;
use std::convert::{TryFrom, TryInto};

pub(crate) fn get_user_coins(user: &User) -> u64 {
	u64::from_be_bytes(
		DB.get(user.id.0.to_be_bytes())
			.unwrap()
			.unwrap_or_else(|| {
				IVec::from(&(100 as u64).to_be_bytes())
			})
			.to_vec()
			.try_into()
			.unwrap(),
	)
}

pub(crate) fn set_user_coins(id: u64, coins: u64) {
	DB.insert(id.to_be_bytes(), IVec::from(&(coins).to_be_bytes()))
		.unwrap();
}

pub(crate) async fn get_leaderboard(
	guild: &Guild,
	ctx: &(impl CacheHttp + Copy),
) -> impl Iterator<Item = (String, String, bool)> {
	let records = DB
		.iter()
		.map(|r| r.unwrap())
		.map(|(id, balance)| {
			(
				u64::from_be_bytes(id.to_vec().try_into().unwrap()),
				u64::from_be_bytes(
					balance.to_vec().try_into().unwrap(),
				),
			)
		})
		.sorted_by(|a, b| b.1.cmp(&a.1));

	let users = stream::iter(records)
		.enumerate()
		.then(async move |(index, (id, balance))| {
			(
				format!("#{}.", index + 1),
				format!(
					"{}: `{}`",
					get_user_nick(
						guild,
						ctx,
						&UserId::try_from(id)
							.unwrap()
							.to_user(ctx)
							.await
							.unwrap(),
					)
					.await,
					balance,
				),
				false,
			)
		})
		.collect::<Vec<_>>()
		.await;

	users.into_iter().take(5)
}
