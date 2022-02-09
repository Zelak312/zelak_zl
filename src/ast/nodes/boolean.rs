use crate::ast::node_box::NodeBox;

pub struct NBoolean {
    pub val: bool,
}

impl NBoolean {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "Boolean");
        println!("{}val: {}", "\t".repeat(tab + 1), self.val);
    }
}
