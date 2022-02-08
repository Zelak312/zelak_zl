use crate::ast::node_box::NodeBox;

pub struct NConditionStatement {
    pub operator: String,
    pub left: Box<NodeBox>,
    pub right: Box<NodeBox>,
}

impl NConditionStatement {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "ConditionStatement");
        println!("{}operator: {}", "\t".repeat(tab + 1), self.operator);
        self.left.debug(Some(tab + 2));
        self.right.debug(Some(tab + 2));
    }
}
