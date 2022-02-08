pub struct NIdentifier {
    pub name: String,
}

impl NIdentifier {
    pub fn debug(self, tab: usize) {
        println!("{}{}", "\t".repeat(tab), "Identifier");
        println!("{}name: {}", "\t".repeat(tab + 1), self.name);
    }
}
