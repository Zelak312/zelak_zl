use crate::ast::node_box::NodeBox;

pub struct NCallStatement {
    pub identifier: Box<NodeBox>,
    pub parameters: Vec<Box<NodeBox>>,
}

impl NCallStatement {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "CallStatement");
        self.identifier.debug(Some(tab + 1));
        println!("{}{}", "\t".repeat(tab + 1), "parameters: ");
        for param in &self.parameters {
            param.debug(Some(tab + 2));
        }
    }
}
