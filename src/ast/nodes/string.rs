pub struct NString {
    pub val: String,
}

impl NString {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "String");
        println!("{}val: {}", "\t".repeat(tab + 1), self.val);
    }
}
