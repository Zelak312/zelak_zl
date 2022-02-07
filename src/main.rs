mod baseParser;
mod bash_nodes;
mod generator;
mod lexer;
mod nodes;
mod parser;
mod token;
mod transpiler;

use generator::generate_code;
use lexer::Lexer;
use parser::Parser;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use transpiler::transpile;

use crate::token::Type;

fn debug_lexer(line: &str) {
    let mut lexer = Lexer::new(line);
    let mut token = lexer.get_next();
    while token._type != Type::EOL {
        println!("{:?}", token);
        token = lexer.get_next();
    }
}

fn main() {
    let contents = fs::read_to_string("test.zl").expect("Something went wrong reading the file");
    let lexer = Lexer::new(&contents);
    let ast = Parser::gen_ast(lexer);
    let bash_ast = transpile(ast);
    let code = generate_code(bash_ast);

    let path = Path::new("transpiled.sh");
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    match file.write_all(code.as_bytes()) {
        Err(why) => panic!("couldn't write to {}", why),
        Ok(_) => println!("successfully wrote"),
    }
}
