use crate::ast::node_box::NodeBox;

pub struct BStringConcatExpression {
    pub content: Box<NodeBox>,
}

impl BStringConcatExpression {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "StringConcatExpression");
        self.content.debug(Some(tab + 1))
    }
}
