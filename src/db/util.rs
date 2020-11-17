use crate::DB;

use serenity::model::user::User;
use sled::IVec;
use std::{convert::TryInto, sync::Arc};

pub fn get_user_coins(user: Arc<&User>) -> u64 {
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

pub fn set_user_coins(id: u64, coins: u64) {
	DB.insert(id.to_be_bytes(), IVec::from(&(coins).to_be_bytes()))
		.unwrap();
}
