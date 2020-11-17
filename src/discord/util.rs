use serenity::{
	http::CacheHttp,
	model::{guild::Guild, user::User},
};
use std::sync::Arc;

pub async fn get_user_nick<'a>(
	guild: Arc<Guild>,
	ctx: impl CacheHttp,
	user: Arc<&'a User>,
) -> String {
	guild
		.member(ctx, user.id)
		.await
		.unwrap()
		.nick
		.unwrap_or(user.name.clone())
}
