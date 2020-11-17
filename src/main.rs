#![feature(async_closure)]

extern crate dotenv;
extern crate lazysort;

use anyhow::{Context as _, Result};
use colored::Colorize;
use commands::{all::*, get::*, give::*, ping::*};
use dotenv::dotenv;
use lazy_static::lazy_static;
use serenity::{
	async_trait,
	client::{Client, EventHandler},
	framework::standard::{macros::group, StandardFramework},
	model::gateway::Ready,
	prelude::*,
};
use std::env;

#[macro_use]
mod log;
#[macro_use]
mod discord;
mod commands;
mod db;

lazy_static! {
	static ref LOG: bool = env::var_os("LOG").is_some();
	static ref DB: sled::Db = sled::open("bors.db").unwrap();
}

#[group]
#[commands(ping, get, give, all)]
struct General;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		success!("Connected as {}!", ready.user.name);
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let framework = StandardFramework::new()
		.configure(|c| c.prefix("~"))
		.group(&GENERAL_GROUP);

	let token = env::var("TOKEN").context(
		"Failed to find get Discord Token from envioronment \
		 variable!",
	)?;
	success!("Read bot token!");

	let mut client = Client::builder(token)
		.event_handler(Handler)
		.framework(framework)
		.await
		.context("Failed to create bot client!")?;
	success!("Started bot client!");

	client
		.start()
		.await
		.context("Failed to start bot client!")?;

	Ok(())
}
