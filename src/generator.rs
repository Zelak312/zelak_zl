use std::any::{Any, TypeId};

use crate::bash_nodes::{
    bassignment::BAssignment, becho::BEcho, bexpr::BExpr, biden::BIden, bnumber::BNumber,
    bprogram::BProgram, bstring::BString,
};

pub fn generate_code(root: Box<dyn Any>) -> Box<String> {
    let mut str = String::new();
    let actual_id = (&*root).type_id();
    if actual_id == TypeId::of::<BProgram>() {
        let bash_program = root.downcast::<BProgram>().unwrap();
        str += "#!/bin/bash\n";
        str += &generate_code(bash_program.b_expr);
    } else if actual_id == TypeId::of::<BExpr>() {
        let b_expr = root.downcast::<BExpr>().unwrap();
        for child in b_expr.childs {
            str += &(generate_code(child).to_string() + "\n");
        }
    } else if actual_id == TypeId::of::<BEcho>() {
        let b_echo = root.downcast::<BEcho>().unwrap();
        str += &("echo ".to_string() + &generate_code(b_echo.to_echo));
    } else if actual_id == TypeId::of::<BAssignment>() {
        let b_assignment = root.downcast::<BAssignment>().unwrap();
        str += &(b_assignment.iden + "=" + &generate_code(b_assignment.content));
    } else if actual_id == TypeId::of::<BIden>() {
        let b_iden = root.downcast::<BIden>().unwrap();
        str += &("$".to_string() + &b_iden.name);
    } else if actual_id == TypeId::of::<BString>() {
        let b_string = root.downcast::<BString>().unwrap();
        str += &("\"".to_string() + &b_string.val + "\"");
    } else if actual_id == TypeId::of::<BNumber>() {
        let b_number = root.downcast::<BNumber>().unwrap();
        str += &b_number.val.to_string();
    }

    return Box::new(str);
}
