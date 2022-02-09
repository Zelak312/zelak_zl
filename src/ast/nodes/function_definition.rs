use crate::ast::node_box::NodeBox;

pub struct NFunctionDefinition {
    pub identifier: Box<NodeBox>,
    pub arguments: Vec<Box<NodeBox>>,
    pub expressions: Vec<Box<NodeBox>>,
}

impl NFunctionDefinition {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "FunctionDefinition");
        self.identifier.debug(Some(tab + 1));
        println!("{}{}", "\t".repeat(tab + 1), "arguments: ");
        for argument in &self.arguments {
            argument.debug(Some(tab + 2));
        }
        println!("{}{}", "\t".repeat(tab + 1), "expressions: ");
        for expression in &self.expressions {
            expression.debug(Some(tab + 2));
        }
    }
}
