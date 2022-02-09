use crate::ast::node_box::NodeBox;

pub struct NArray {
    pub items: Vec<Box<NodeBox>>,
}

impl NArray {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "Array");
        println!("{}{}", "\t".repeat(tab + 1), "items: ");
        for item in &self.items {
            item.debug(Some(tab + 2));
        }
    }
}
