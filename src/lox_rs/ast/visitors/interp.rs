use std::sync::Mutex;

use anyhow::{bail, Result};
use once_cell::sync::Lazy;

use super::super::super::{
	ast::{
		expr::{Expr, Literal},
		stmt::Stmt,
	},
	env::Env,
	lexer::tokens::token_type::Operator,
};

#[derive(Default, Debug)]
pub struct Interperter {
	env: Env,
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
							Literal::String(s.repeat(f64::round(n) as usize))
						}
						(Literal::String(s), Literal::Number(n)) => {
							Literal::String(s.repeat(f64::round(n) as usize))
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
						Literal::Number(n) => Literal::Number(f64::abs(n)),
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
					},
					Operator::Not => match right {
						Literal::Number(n) => Literal::Boolean(n != 0.),
						Literal::String(s) => Literal::Boolean(!s.is_empty()),
						Literal::Boolean(b) => Literal::Boolean(!b),
						Literal::Null => Literal::Boolean(true),
					},
					other => bail!("Should not get {:?} as an unary operator", &other),
				};
				Ok(new_lit)
			}
			Expr::Variable(name) => self.env.get(name).cloned(),
			Expr::Assign(name, value) => {
				let value = self.expr(*value)?;
				self.env.redefine(name, value)?;

				Ok(Literal::Null)
			}
		}
	}
	pub fn stmt(&mut self, stmt: Stmt) -> Result<Literal> {
		log::debug!("{:?}", &self.env);

		match stmt {
			Stmt::Expression(e) => self.expr(e),
			Stmt::Print(e) => {
				let result = self.expr(e)?;
				println!("{:?}", &result);

				Ok(Literal::Null)
			}
			Stmt::Var(name, value) => {
				if let Some(value) = value {
					let value = self.expr(value)?;
					self.env.define(name, value.clone());

					Ok(value)
				} else {
					self.env.define(name, Literal::Null);
					Ok(Literal::Null)
				}
			}
			Stmt::Block(statements) => {
				let prev_env = self.env.clone();
				self.env = Env::new(Box::new(prev_env));

				for statement in statements {
					self.stmt(statement)?;
				}
				self.env = *self.env.get_parent().unwrap();

				Ok(Literal::Null)
			}
			Stmt::If(cond, then_block, else_block) => {
				if self.expr(cond)?.is_truthy() {
					self.stmt(*then_block)?;
				} else if let Some(else_block) = else_block {
					self.stmt(*else_block)?;
				}
				Ok(Literal::Null)
			}
		}
	}
}

pub static INTERPERTER: Lazy<Mutex<Interperter>> = Lazy::new(|| Mutex::new(Interperter::default()));