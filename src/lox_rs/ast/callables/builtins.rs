use std::time::{SystemTime, UNIX_EPOCH};

use super::{super::expr::Literal, callable::Callable};

pub const CLOCK: Callable = Callable {
	arity: 0,
	as_str: "<native fn>",
	func: |_| {
		let seconds = SystemTime::now().duration_since(UNIX_EPOCH)?;
		let seconds = seconds.as_secs_f64();
		Ok(Literal::Number(seconds))
	},
};
