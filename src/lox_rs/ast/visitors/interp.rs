use std::sync::Mutex;

use anyhow::{bail, Result};
use once_cell::sync::Lazy;

use super::super::{
	super::{
		ast::{
			expr::{Expr, Literal},
			stmt::Stmt,
		},
		env::Env,
		lexer::tokens::token_type::Operator,
	},
	callables::builtins::CLOCK,
};
use crate::lox_rs::ast::callables::callable::Callable;

#[derive(Debug)]
pub struct Interperter {
	pub global: Env,
	pub local: Env,
}
impl Interperter {
	pub fn expr(&mut self, expr: Expr) -> Result<Literal> {
		match expr {
			Expr::Binary(left, op, right) => {
				let left = self.expr(*left)?;
				let right = self.expr(*right)?;

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
			Expr::Grouping(expr) => self.expr(*expr),
			Expr::Literal(lit) => Ok(lit),
			Expr::Unary(op, right) => {
				let right = self.expr(*right)?;

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
						Literal::Function(_) => bail!("Can't add a function"),
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
						Literal::Function(_) => bail!("Can't sub a function"),
					},
					Operator::Not => match right {
						Literal::Number(n) => Literal::Boolean(n != 0.),
						Literal::String(s) => Literal::Boolean(!s.is_empty()),
						Literal::Boolean(b) => Literal::Boolean(!b),
						Literal::Null => Literal::Boolean(true),
						Literal::Function(_) => Literal::Boolean(false),
					},
					other => bail!("Should not get {:?} as an unary operator", &other),
				};
				Ok(new_lit)
			}
			Expr::Variable(name) => self
				.local
				.get(name.clone())
				.or_else(|_| self.global.get(name))
				.cloned(),
			Expr::Assign(name, value) => {
				let value = self.expr(*value)?;
				self.local.redefine(name, value.clone())?;

				Ok(value)
			}
			Expr::Logical(lhs, op, rhs) => {
				let lhs = self.expr(*lhs)?;
				match (op, lhs.is_truthy()) {
					(Operator::Or, true) => Ok(lhs),
					(Operator::Or, false) => self.expr(*rhs),
					(Operator::And, true) => self.expr(*rhs),
					(Operator::And, false) => Ok(lhs),
					(other, _) => bail!("Invalid logical operator recieved {:?}", other),
				}
			}
			Expr::Call(callee, _paren, args) => {
				let callee = self.expr(*callee)?;
				let args_as_lit: Vec<Literal> =
					args.iter().map(|a| self.expr(a.clone()).unwrap()).collect();

				let Literal::Function(func) = &callee else {
					bail!("Unexpected type for the callee, {:?}", &callee);
				};
				func.call(self, args_as_lit)
			}
		}
	}
	pub fn stmt(&mut self, stmt: Stmt) -> Result<Literal> {
		log::debug!("{:?}", &self.local);

		match stmt {
			Stmt::Expression(e) => self.expr(e),
			Stmt::Print(e) => {
				let result = self.expr(e)?;
				println!("{:?}", &result);

				Ok(result)
			}
			Stmt::Var(name, value) => {
				if let Some(value) = value {
					let value = self.expr(value)?;
					self.local.define(name, value.clone());

					Ok(value)
				} else {
					self.local.define(name, Literal::Null);
					Ok(Literal::Null)
				}
			}
			Stmt::Block(statements) => {
				let prev_env = self.local.clone();
				self.local = Env::new(Box::new(prev_env));

				let mut result = Literal::Null;
				for statement in statements {
					result = self.stmt(statement)?;
				}
				self.local = *self.local.get_parent().unwrap();

				Ok(result)
			}
			Stmt::If(cond, then_block, else_block) => {
				let mut result = Literal::Null;
				if self.expr(cond)?.is_truthy() {
					result = self.stmt(*then_block)?;
				} else if let Some(else_block) = else_block {
					result = self.stmt(*else_block)?;
				}
				Ok(result)
			}
			Stmt::While(cond, block) => {
				let mut result = Literal::Null;
				while self.expr(cond.clone())?.is_truthy() {
					result = self.stmt(*block.clone())?;
				}
				Ok(result)
			}
			Stmt::For(initializer, condition, increment, block) => {
				let mut init_param_name: Option<String> = None;
				if let Some(initializer) = initializer {
					if let Stmt::Var(ref name, _) = *initializer {
						init_param_name = Some(name.clone());
						self.stmt(*initializer)?;
					} else {
						bail!("This should be a loop interator initializer")
					}
				}

				let block = Box::new(Stmt::Block({
					let mut stmts: Vec<Stmt> = Vec::new();

					stmts.push(*block);
					if let Some(increment) = increment {
						stmts.push(Stmt::Expression(increment));
					}
					stmts
				}));
				let mut result = Literal::Null;
				while condition
					.clone()
					.map_or_else(|| Ok(Literal::Null), |expr| self.expr(expr))?
					.is_truthy()
				{
					let block = block.clone();
					result = self.stmt(*block)?;
				}
				if let Some(init_param_name) = init_param_name {
					self.local.remove(&init_param_name);
				}

				Ok(result)
			}
			Stmt::Function(name, inputs, block) => {
				let inputs: Vec<_> = inputs.iter().map(|p| self.expr(p.clone())).collect();
				let block = self.stmt(*block)?;

				let fn_str = format!(
					r"function {name}({inputs:?}) {{
                        {block}
                    }}"
				);

				self.local.define(
					name,
					Literal::Function(Callable::new(inputs.len(), fn_str, |_inputs| {
						Ok(Literal::Null)
					})),
				);

				Ok(Literal::Null)
			}
		}
	}
}

impl Default for Interperter {
	fn default() -> Self {
		let global = {
			let mut g = Env::default();
			g.define(
				"clock".to_owned(),
				Literal::Function(CLOCK.lock().unwrap().to_owned()),
			);
			g
		};
		Self {
			global,
			local: Env::default(),
		}
	}
}

pub static INTERPERTER: Lazy<Mutex<Interperter>> = Lazy::new(|| Mutex::new(Interperter::default()));
