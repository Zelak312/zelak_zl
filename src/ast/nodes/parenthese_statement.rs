use crate::ast::node_box::NodeBox;

pub struct NParentheseStatement {
    pub content: Box<NodeBox>,
}

impl NParentheseStatement {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "ParentheseStatement");
        self.content.debug(Some(tab + 1));
    }
}
