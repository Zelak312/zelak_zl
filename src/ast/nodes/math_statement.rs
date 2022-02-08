use crate::ast::node_box::NodeBox;

pub struct NMathStatement {
    pub operator: String,
    pub left: Box<NodeBox>,
    pub right: Box<NodeBox>,
}

impl NMathStatement {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "MathStatement");
        println!("{}operator: {}", "\t".repeat(tab + 1), self.operator);
        self.left.debug(Some(tab + 1));
        self.right.debug(Some(tab + 1));
    }
}
