use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

use super::{super::expr::Literal, native_fn::NativeFn};

pub const NOW: NativeFn = NativeFn::new(0, |_inputs: Vec<Literal>| -> Result<Literal> {
	let seconds = SystemTime::now().duration_since(UNIX_EPOCH)?;
	let seconds = seconds.as_secs_f64();
	Ok(Literal::Number(seconds))
});
