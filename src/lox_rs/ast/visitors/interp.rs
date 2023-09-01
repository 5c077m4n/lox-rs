use anyhow::{bail, Result};

use super::super::super::{
	ast::expr::{Expr, Literal, Stmt},
	lexer::tokens::token_type::Operator,
};

pub fn interpret_expr(expr: &Expr) -> Result<Literal> {
	match expr {
		Expr::Binary(left, op, right) => {
			let left = interpret_expr(left)?;
			let right = interpret_expr(right)?;

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
		Expr::Grouping(expr) => interpret_expr(expr),
		Expr::Literal(lit) => Ok(lit.clone()),
		Expr::Unary(op, right) => {
			let right = interpret_expr(right)?;

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
					Literal::Boolean(b) => {
						if b {
							Literal::Number(1.)
						} else {
							Literal::Number(0.)
						}
					}
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
					Literal::Boolean(b) => {
						if b {
							Literal::Number(-1.)
						} else {
							Literal::Number(0.)
						}
					}
					Literal::Null => Literal::Number(0.),
				},
				Operator::Not => match right {
					Literal::Number(n) => {
						if n == 0. {
							Literal::Boolean(true)
						} else {
							Literal::Boolean(false)
						}
					}
					Literal::String(s) => {
						if s.is_empty() {
							Literal::Boolean(true)
						} else {
							Literal::Boolean(false)
						}
					}
					Literal::Boolean(b) => {
						if b {
							Literal::Boolean(false)
						} else {
							Literal::Boolean(true)
						}
					}
					Literal::Null => Literal::Boolean(true),
				},
				other => unreachable!("Should not get {:?} as an unary op", &other),
			};
			Ok(new_lit)
		}
	}
}

pub fn interpret_stmt(stmt: &Stmt) -> Result<Literal> {
	match stmt {
		Stmt::Expression(e) => interpret_expr(e),
		Stmt::Print(e) => {
			let result = interpret_expr(e)?;
			println!("{:?}", &result);

			Ok(Literal::Null)
		}
	}
}
