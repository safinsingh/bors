use serenity::{
	http::CacheHttp,
	model::{guild::Guild, user::User},
};

pub async fn get_user_nick(
	guild: &Guild,
	ctx: impl CacheHttp,
	user: &User,
) -> String {
	guild
		.member(ctx, user.id)
		.await
		.unwrap()
		.nick
		.unwrap_or_else(|| user.name.clone())
}
