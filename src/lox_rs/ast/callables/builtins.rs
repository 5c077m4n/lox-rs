use std::{
	sync::Mutex,
	time::{SystemTime, UNIX_EPOCH},
};

use once_cell::sync::Lazy;

use super::{super::expr::Literal, callable::Callable};

pub static CLOCK: Lazy<Mutex<Callable>> = Lazy::new(|| {
	let func = Callable::new(0, "<native fn>".to_string(), |_| {
		let seconds = SystemTime::now().duration_since(UNIX_EPOCH)?;
		let seconds = seconds.as_secs_f64();
		Ok(Literal::Number(seconds))
	});
	Mutex::new(func)
});
