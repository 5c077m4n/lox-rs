use anyhow::{bail, Result};

use super::super::{
	super::{
		ast::{
			expr::{Expr, Literal},
			stmt::Stmt,
		},
		env::Env,
		lexer::tokens::token_type::Operator,
	},
	callables::{builtins::NOW, callable::Callable, custom_fn::CustomFn},
};

#[derive(Debug)]
pub struct Interperter {
	pub global: Env,
	pub local: Env,
	return_value: Option<Literal>,
}
impl Interperter {
	pub fn expr(&mut self, expr: &Expr) -> Result<Literal> {
		match expr {
			Expr::Binary(left, op, right) => {
				let left = self.expr(left)?;
				let right = self.expr(right)?;

				let new_lit = match op {
					Operator::NotEq => Literal::Boolean(left != right),
					Operator::EqEq => Literal::Boolean(left == right),
					Operator::Gt => match (left, right) {
						(Literal::Number(n1), Literal::Number(n2)) => Literal::Boolean(n1 > n2),
						other => bail!("Please only compair numbers, but got {:?}", &other),
					},
					Operator::Gte => match (left, right) {
						(Literal::Number(n1), Literal::Number(n2)) => Literal::Boolean(n1 >= n2),
						other => bail!("Please only compair numbers, but got {:?}", &other),
					},
					Operator::Lt => match (left, right) {
						(Literal::Number(n1), Literal::Number(n2)) => Literal::Boolean(n1 < n2),
						other => bail!("Please only compair numbers, but got {:?}", &other),
					},
					Operator::Lte => match (left, right) {
						(Literal::Number(n1), Literal::Number(n2)) => Literal::Boolean(n1 <= n2),
						other => bail!("Please only compair numbers, but got {:?}", &other),
					},
					Operator::Add => match (left, right) {
						(Literal::Number(n1), Literal::Number(n2)) => Literal::Number(n1 + n2),
						(Literal::String(s1), Literal::String(s2)) => Literal::String(s1 + &s2),
						other => bail!(
							"Please only add number to number or string to string, not {:?}",
							&other
						),
					},
					Operator::Sub => match (left, right) {
						(Literal::Number(n1), Literal::Number(n2)) => Literal::Number(n1 - n2),
						other => bail!(
							"Please only subtract a number from a number, not {:?}",
							&other
						),
					},
					Operator::Mul => match (left, right) {
						(Literal::Number(n1), Literal::Number(n2)) => Literal::Number(n1 * n2),
						(Literal::Number(n), Literal::String(s)) => {
							Literal::String(s.repeat(n.round() as usize))
						}
						(Literal::String(s), Literal::Number(n)) => {
							Literal::String(s.repeat(n.round() as usize))
						}
						other => bail!(
							"Please only multiply number to number and string to number, not {:?}",
							&other
						),
					},
					Operator::Div => match (left, right) {
						(Literal::Number(n1), Literal::Number(n2)) => Literal::Number(n1 / n2),
						other => bail!(
							"Please only divide a number from a number, not {:?}",
							&other
						),
					},
					other => bail!("Should not get {:?} as an binary op", &other),
				};
				Ok(new_lit)
			}
			Expr::Grouping(expr) => self.expr(expr),
			Expr::Literal(lit) => Ok(lit.clone()),
			Expr::Unary(op, right) => {
				let right = self.expr(right)?;

				let new_lit = match op {
					Operator::Add => match right {
						Literal::Number(n) => Literal::Number(n.abs()),
						Literal::String(s) => {
							if s.is_empty() {
								Literal::Number(0.)
							} else {
								match s.parse::<f64>() {
									Ok(num) => Literal::Number(num),
									Err(_) => Literal::Null,
								}
							}
						}
						Literal::Boolean(b) => Literal::Number(if b { 1. } else { 0. }),
						Literal::Null => Literal::Number(0.),
						Literal::NativeFunction(_) | Literal::CustomFunction(_) => {
							bail!("Can't add a function")
						}
					},
					Operator::Sub => match right {
						Literal::Number(n) => Literal::Number(n * (-1.)),
						Literal::String(s) => {
							if s.is_empty() {
								Literal::Number(0.)
							} else {
								match s.parse::<f64>() {
									Ok(num) => Literal::Number(-num),
									Err(_) => Literal::Null,
								}
							}
						}
						Literal::Boolean(b) => Literal::Number(if b { -1. } else { 0. }),
						Literal::Null => Literal::Number(0.),
						Literal::NativeFunction(_) | Literal::CustomFunction(_) => {
							bail!("Can't sub a function")
						}
					},
					Operator::Not => match right {
						Literal::Number(n) => Literal::Boolean(n != 0.),
						Literal::String(s) => Literal::Boolean(!s.is_empty()),
						Literal::Boolean(b) => Literal::Boolean(!b),
						Literal::Null => Literal::Boolean(true),
						Literal::NativeFunction(_) | Literal::CustomFunction(_) => {
							Literal::Boolean(false)
						}
					},
					other => bail!("Should not get {:?} as an unary operator", &other),
				};
				Ok(new_lit)
			}
			Expr::Variable(name) => self
				.local
				.get(name.clone())
				.or_else(|_| self.global.get(name.to_string()))
				.cloned(),
			Expr::Assign(name, value) => {
				let value = self.expr(value)?;
				self.local.redefine(name.to_string(), value.clone())?;

				Ok(value)
			}
			Expr::Logical(lhs, op, rhs) => {
				let lhs = self.expr(lhs)?;
				match (op, lhs.is_truthy()) {
					(Operator::Or, true) => Ok(lhs),
					(Operator::Or, false) => self.expr(rhs),
					(Operator::And, true) => self.expr(rhs),
					(Operator::And, false) => Ok(lhs),
					(other, _) => bail!("Invalid logical operator recieved {:?}", other),
				}
			}
			Expr::Call(callee, _paren, args) => {
				let callee = self.expr(callee)?;
				let args = args.to_vec();

				match &callee {
					Literal::NativeFunction(func) => func.call(self, args),
					Literal::CustomFunction(func) => func.call(self, args),
					other => bail!("Unexpected type for the callee, {:?}", other),
				}
			}
		}
	}
	fn stmt(&mut self, stmt: &Stmt) -> Result<Literal> {
		match stmt {
			Stmt::Expression(e) => self.expr(e),
			Stmt::Print(e) => {
				let result = self.expr(e)?;
				println!("{}", &result);

				Ok(result)
			}
			Stmt::Var(name, value) => {
				if let Some(value) = value {
					let value = self.expr(value)?;
					self.local.define(name.to_string(), value.clone());

					Ok(value)
				} else {
					self.local.define(name.to_string(), Literal::Null);
					Ok(Literal::Null)
				}
			}
			Stmt::Block(statements) => {
				let prev_env = self.local.clone();
				self.local = Env::new(Box::new(prev_env));

				let mut result = Literal::Null;
				for statement in statements {
					if let Some(lit) = &self.return_value {
						let lit = lit.clone();
						self.return_value = None;

						return Ok(lit);
					} else {
						result = self.stmt(statement)?;
					}
				}
				self.local = *self.local.get_parent().unwrap();

				Ok(result)
			}
			Stmt::If(cond, then_block, else_block) => {
				let mut result = Literal::Null;
				if self.expr(cond)?.is_truthy() {
					result = self.stmt(then_block)?;
				} else if let Some(else_block) = else_block {
					result = self.stmt(else_block)?;
				}
				Ok(result)
			}
			Stmt::While(cond, block) => {
				let mut result = Literal::Null;
				while self.expr(cond)?.is_truthy() {
					result = self.stmt(block)?;
				}
				Ok(result)
			}
			Stmt::For(initializer, condition, increment, block) => {
				let mut init_param_name: Option<String> = None;
				if let Some(initializer) = initializer {
					if let Stmt::Var(ref name, _) = **initializer {
						init_param_name = Some(name.clone());
						self.exec(initializer)?;
					} else {
						bail!("This should be a loop interator initializer")
					}
				}

				let block = Box::new(Stmt::Block({
					let mut stmts: Vec<Stmt> = Vec::new();

					stmts.push(*block.clone());
					if let Some(increment) = increment {
						stmts.push(Stmt::Expression(increment.clone()));
					}
					stmts
				}));
				let mut result = Literal::Null;
				while condition
					.clone()
					.map_or_else(|| Ok(Literal::Null), |expr| self.expr(&expr))?
					.is_truthy()
				{
					let block = block.clone();
					result = self.stmt(&block)?;
				}
				if let Some(init_param_name) = init_param_name {
					self.local.remove(&init_param_name);
				}

				Ok(result)
			}
			Stmt::Function(name, inputs, block) => {
				let custom_fn = Literal::CustomFunction(CustomFn::new(
					name.to_string(),
					inputs.to_vec(),
					block.clone(),
				));
				self.local.define(name.clone(), custom_fn.clone());

				Ok(custom_fn)
			}
			Stmt::Return(value) => {
				let ret_val = self.expr(value)?;
				self.return_value = Some(ret_val.clone());

				Ok(ret_val)
			}
		}
	}

	pub fn exec(&mut self, stmt: &Stmt) -> Result<Literal> {
		if let Some(lit) = &self.return_value {
			let lit = lit.clone();
			self.return_value = None;

			Ok(lit)
		} else {
			self.stmt(stmt)
		}
	}
	pub fn exec_block(&mut self, block: &Stmt, env: Box<Env>) -> Result<Literal> {
		let prev_env = self.local.clone();
		self.local = Env::new(env);

		let result = self.exec(block);
		self.local = prev_env;

		result
	}
}

impl Default for Interperter {
	fn default() -> Self {
		let global = {
			let mut g = Env::default();
			g.define(NOW.get_name(), Literal::NativeFunction(NOW));
			g
		};
		Self {
			global,
			local: Env::default(),
			return_value: None,
		}
	}
}
