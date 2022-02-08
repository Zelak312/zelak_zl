pub struct NNumber {
    pub val: f64,
}

impl NNumber {
    pub fn debug(&self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "Number");
        println!("{}val: {}", "\t".repeat(tab + 1), self.val);
    }
}
