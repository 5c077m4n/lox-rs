use super::node_types::{Binary, Grouping, Unary};

pub trait Visitor {
	fn visit_binary(&self, bin: &Binary);
	fn visit_grouping(&self, group: &Grouping);
	fn visit_unary(&self, unary: &Unary);
}
