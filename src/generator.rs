use std::any::{Any, TypeId};

use crate::{
    bash_nodes::{
        becho::BEcho, bmath_expr::BMathExpr, bprogram::BProgram, bvariable_statement::BAssignment,
    },
    nodes::{bin_op::BinOp, identifier::Iden, number::Number, program::Expr, string::NString},
};

pub fn generate_code(root: Box<dyn Any>) -> Box<String> {
    let mut str = String::new();
    let actual_id = (&*root).type_id();
    if actual_id == TypeId::of::<BProgram>() {
        let bash_program = root.downcast::<BProgram>().unwrap();
        str += "#!/bin/bash\n";
        str += &generate_code(bash_program.b_expr);
    } else if actual_id == TypeId::of::<Expr>() {
        let b_expr = root.downcast::<Expr>().unwrap();
        for child in b_expr.childs {
            str += &(generate_code(child).to_string() + "\n");
        }
    } else if actual_id == TypeId::of::<BEcho>() {
        let mut b_echo = root.downcast::<BEcho>().unwrap();
        if b_echo.to_echo.len() == 1 {
            str += &("echo ".to_string() + &generate_code(b_echo.to_echo.remove(0)));
        } else {
            str += &("printf \"".to_string() + &"%s, ".repeat(b_echo.to_echo.len() - 1));
            str += &("%s\\n\"");
            for param in b_echo.to_echo {
                str += &(" ".to_string() + &generate_code(param));
            }
        }
    } else if actual_id == TypeId::of::<BAssignment>() {
        let b_assignment = root.downcast::<BAssignment>().unwrap();
        str += &(b_assignment.iden + "=" + &generate_code(b_assignment.content));
    } else if actual_id == TypeId::of::<BMathExpr>() {
        let b_math_expr = root.downcast::<BMathExpr>().unwrap();
        str += &("$(expr ".to_string() + &generate_code(b_math_expr.content) + ")");
    } else if actual_id == TypeId::of::<BinOp>() {
        let b_op = root.downcast::<BinOp>().unwrap();
        let mid = &(generate_code(b_op.left).to_string()
            + " "
            + match b_op.op.as_str() {
                "*" => "\\*",
                _ => b_op.op.as_str(),
            }
            + " "
            + &generate_code(b_op.right));
        if b_op.parenthese {
            str += &("(".to_string() + mid + ")")
        } else {
            str += mid;
        }
    } else if actual_id == TypeId::of::<Iden>() {
        let b_iden = root.downcast::<Iden>().unwrap();
        str += &("$".to_string() + &b_iden.name);
    } else if actual_id == TypeId::of::<NString>() {
        let b_string = root.downcast::<NString>().unwrap();
        str += &("\"".to_string() + &b_string.val + "\"");
    } else if actual_id == TypeId::of::<Number>() {
        let b_number = root.downcast::<Number>().unwrap();
        str += &b_number.val.to_string();
    }

    return Box::new(str);
}
