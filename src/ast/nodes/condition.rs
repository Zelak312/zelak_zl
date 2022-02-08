use crate::ast::node_box::NodeBox;

pub struct NCondition {
    pub operator: String,
    pub left: Box<NodeBox>,
    pub right: Box<NodeBox>,
}

impl NCondition {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "Condition");
        println!("{}operator: {}", "\t".repeat(tab + 1), self.operator);
        self.left.debug(Some(tab + 2));
        self.right.debug(Some(tab + 2));
    }
}
