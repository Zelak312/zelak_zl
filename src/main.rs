mod baseParser;
mod bash_nodes;
mod lexer;
mod nodes;
mod parser;
mod token;

use lexer::Lexer;
use nodes::{
    expression_statement::NExpressionStatement,
    identifier::NIdentifier,
    node_kind::{NodeBox, NodeKind},
    number::NNumber,
    program::NProgram,
    variable_statement::NVariableStatement,
};
use parser::Parser;
use std::fs::{self};

use crate::token::Type;

fn debug_lexer(line: &str) {
    let mut lexer = Lexer::new(line);
    let mut token = lexer.get_next();
    while token._type != Type::EOL {
        println!("{:?}", token);
        token = lexer.get_next();
    }
}

fn debug_ast(root: Box<NodeBox>, tab: usize) -> String {
    let mut l = "\t".repeat(tab);
    match root._type {
        NodeKind::Program => {
            l += "Program\n";
            let prog = root.content.downcast::<NProgram>().unwrap();
            for child in prog.childs {
                l += &debug_ast(child, tab + 1);
            }
        }
        NodeKind::VariableStatement => {
            l += "VariableStatement\n";
            let var = root.content.downcast::<NVariableStatement>().unwrap();
            l += &("\t".repeat(tab + 1) + &format!("declare: {:?}", var.declare_type) + "\n");
            l += &debug_ast(var.identifier, tab + 1);
            l += &debug_ast(var.expression, tab + 1);
        }
        NodeKind::ExpressionStatement => {
            l += "ExpressionStatement\n";
            let expr = root.content.downcast::<NExpressionStatement>().unwrap();
            l += &debug_ast(expr.content, tab + 1);
        }
        NodeKind::Identifier => {
            l += "Identifier\n";
            let iden = root.content.downcast::<NIdentifier>().unwrap();
            l += &("\t".repeat(tab + 1) + &format!("name: {:?}", iden.name) + "\n")
        }
        NodeKind::Number => {
            let num = root.content.downcast::<NNumber>().unwrap();
            l += "Number\n";
            l += &("\t".repeat(tab + 1) + &format!("val: {:?}", num.val) + "\n")
        }
    }

    return l.to_string();
}

fn main() {
    let contents = fs::read_to_string("test.zl").expect("Something went wrong reading the file");
    let lexer = Lexer::new(&contents);
    let ast = Parser::gen_ast(lexer);
    // println!("{}", debug_ast(ast, 0))
    ast.debug(None);
    // let bash_ast = transpile(ast);
    // let code = generate_code(bash_ast);

    // let path = Path::new("transpiled.sh");
    // let mut file = match File::create(&path) {
    //     Err(why) => panic!("couldn't create {}", why),
    //     Ok(file) => file,
    // };

    // match file.write_all(code.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}", why),
    //     Ok(_) => println!("successfully wrote"),
    // }
}
