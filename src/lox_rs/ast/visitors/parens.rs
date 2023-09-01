use super::super::expr::Expr;

pub fn parenthesize(expr: &Expr) -> String {
	match expr {
		Expr::Binary(left, op, right) => {
			let left_str = &parenthesize(left);
			let right_str = &parenthesize(right);
			format!("({} {} {})", left_str, op.to_str(), right_str)
		}
		Expr::Grouping(expr) => {
			let expr_str = &parenthesize(expr);
			format!("(group {})", expr_str)
		}
		Expr::Literal(value) => value.to_string(),
		Expr::Unary(op, right) => {
			let right_str = &parenthesize(right);
			format!("({} {})", op.to_str(), right_str)
		}
		Expr::Variable(name) => format!("(var {})", name),
		Expr::Assign(var_name, value) => {
			let value = &parenthesize(value);
			format!("(assign {} {})", var_name, value)
		}
	}
}
