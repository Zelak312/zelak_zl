use crate::ast::node_box::NodeBox;

pub struct BMathExpression {
    pub content: Box<NodeBox>,
}

impl BMathExpression {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "MathExpression");
        self.content.debug(Some(tab + 1))
    }
}
