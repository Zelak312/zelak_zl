use crate::ast::node_box::NodeBox;

pub struct NProgram {
    pub childs: Vec<Box<NodeBox>>,
}

impl NProgram {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "Program");
        for child in &self.childs {
            child.debug(Some(tab + 1));
        }
    }
}
