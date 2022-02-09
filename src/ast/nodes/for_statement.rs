use crate::ast::node_box::NodeBox;

pub struct NForStatement {
    pub start: Option<Box<NodeBox>>,
    pub condition: Box<NodeBox>,
    pub step: Option<Box<NodeBox>>,
    pub expressions: Vec<Box<NodeBox>>,
}

impl NForStatement {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "ForStatement");
        self.start.as_ref().map_or_else(
            || println!("{}None", "\t".repeat(tab + 1)),
            |n| n.debug(Some(tab + 1)),
        );
        self.condition.debug(Some(tab + 1));
        self.step.as_ref().map_or_else(
            || println!("{}None", "\t".repeat(tab + 1)),
            |n| n.debug(Some(tab + 1)),
        );
        for expression in &self.expressions {
            expression.debug(Some(tab + 2));
        }
    }
}
