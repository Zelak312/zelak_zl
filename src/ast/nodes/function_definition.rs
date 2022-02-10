use crate::ast::node_box::NodeBox;

pub struct NFunctionDefinition {
    pub identifier: String,
    pub arguments: Vec<Box<NodeBox>>,
    pub expressions: Vec<Box<NodeBox>>,
}

impl NFunctionDefinition {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "FunctionDefinition");
        println!("{}identifier: {}", "\t".repeat(tab + 1), self.identifier);
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
