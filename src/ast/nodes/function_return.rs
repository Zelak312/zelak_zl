use crate::ast::node_box::NodeBox;

pub struct NFunctionReturn {
    pub content: Box<NodeBox>,
}

impl NFunctionReturn {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "FunctionReturn");
        self.content.debug(Some(tab + 1));
    }
}
