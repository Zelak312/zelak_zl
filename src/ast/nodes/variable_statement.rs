use crate::{ast::node_box::NodeBox, token::Type};

pub struct NVariableStatement {
    pub declare_type: Option<Type>,
    pub identifier: Box<NodeBox>,
    pub expression: Box<NodeBox>,
}

impl NVariableStatement {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "VariableStatement");
        println!("{}declare: {:?}", "\t".repeat(tab + 1), self.declare_type);
        self.identifier.debug(Some(tab + 1));
        self.expression.debug(Some(tab + 1));
    }
}
