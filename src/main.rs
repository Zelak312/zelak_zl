mod ast;
mod base_parser;
mod bash_nodes;
mod generator;
mod lexer;
mod parser;
mod token;
mod transpiler;
use generator::generate_code;
use lexer::Lexer;
use parser::Parser;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};
use transpiler::transpile;

fn main() {
    let args: Vec<String> = env::args().collect();
    let debug = args.contains(&"debug".to_string());
    let contents = fs::read_to_string("test.zl").expect("Something went wrong reading the file");
    let mut lexer = Lexer::new(&contents);

    if debug {
        lexer.debug();
    }

    let ast = Parser::gen_ast(lexer);
    if debug {
        ast.debug(None);
    }

    let bash_ast = transpile(ast);
    if debug {
        bash_ast.debug(None);
    }
    let code = generate_code(bash_ast, 0, false);

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
