use super::super::expr::Expr;

pub fn parenthesize(expr: &Expr) -> String {
	match expr {
		Expr::Binary(left, op, right) => {
			let left_str = &parenthesize(left);
			let right_str = &parenthesize(right);
			let op = op.to_str();
			format!("({left_str} {op} {right_str})")
		}
		Expr::Grouping(expr) => {
			let expr_str = &parenthesize(expr);
			format!("(group {expr_str})")
		}
		Expr::Literal(value) => value.to_string(),
		Expr::Unary(op, right) => {
			let right_str = &parenthesize(right);
			let op = op.to_str();
			format!("({op} {right_str})")
		}
		Expr::Variable(name) => format!("(var {name})"),
		Expr::Assign(var_name, value) => {
			let value = &parenthesize(value);
			format!("(assign {var_name} {value})")
		}
		Expr::Logical(expr_1, op, expr_2) => {
			let expr_1 = &parenthesize(expr_1);
			let expr_2 = &parenthesize(expr_2);
			let op = op.to_str();
			format!("({op} {expr_1} {expr_2})")
		}
		Expr::Call(callee, _paren, args) => format!("({callee}, {args:?})"),
	}
}
