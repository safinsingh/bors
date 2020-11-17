macro_rules! success_embed {
	($key:literal, $val:expr) => {
		|message_| {
			message_.embed(|embed_| {
				embed_
					.field($key.to_string(), $val.to_string(), true)
					.colour(Colour::from_rgb(0, 255, 0))
				})
			};
	};
}

macro_rules! fail_embed {
	($key:literal, $val:expr) => {
		|message_| {
			message_.embed(|embed_| {
				embed_
					.field($key.to_string(), $val.to_string(), true)
					.colour(Colour::from_rgb(255, 0, 0))
				})
			};
	};
}
