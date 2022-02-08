use crate::ast::node_box::NodeBox;

pub struct NExpressionStatement {
    pub content: Box<NodeBox>,
}

impl NExpressionStatement {
    pub fn debug(self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "ExpressionStatement");
        self.content.debug(Some(tab + 1));
    }
}
