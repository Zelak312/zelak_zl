use crate::ast::node_box::NodeBox;

pub struct NIfStatement {
    pub conditions: Vec<Box<NodeBox>>,
    pub expressions: Vec<Box<NodeBox>>,
}

impl NIfStatement {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "IfStatement");
        println!("{}{}", "\t".repeat(tab + 1), "conditions: ");
        for condition in &self.conditions {
            condition.debug(Some(tab + 2));
        }
        println!("{}{}", "\t".repeat(tab + 1), "expressions: ");
        for expression in &self.expressions {
            expression.debug(Some(tab + 2));
        }
    }
}
