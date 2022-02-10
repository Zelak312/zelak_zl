use crate::{ast::node_box::NodeBox, token::Type};

pub struct NVariableStatement {
    pub declare_type: Option<Type>,
    pub identifier: String,
    pub expression: Box<NodeBox>,
}

impl NVariableStatement {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "VariableStatement");
        println!("{}declare: {:?}", "\t".repeat(tab + 1), self.declare_type);
        println!("{}declare: {}", "\t".repeat(tab + 1), self.identifier);
        self.expression.debug(Some(tab + 1));
    }
}
