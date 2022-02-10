use crate::ast::node_box::NodeBox;

pub struct NStringConcat {
    pub left: Box<NodeBox>,
    pub right: Box<NodeBox>,
}

impl NStringConcat {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "StringConcat");
        self.left.debug(Some(tab + 1));
        self.right.debug(Some(tab + 1));
    }
}
