use std::{
    any::{Any, TypeId},
    vec,
};

use crate::{
    bash_nodes::{
        bassignment::BAssignment, becho::BEcho, bexpr::BExpr, biden::BIden, bnumber::BNumber,
        bprogram::BProgram, bstring::BString,
    },
    zl_nodes::{
        zassignment::ZAssignment, zexpr::ZExpr, zfunction_call::ZFunction_call, ziden::ZIden,
        znumber::ZNumber, zstring::ZString,
    },
};

pub fn transpile(root: Box<ZExpr>) -> Box<BProgram> {
    let b_program = BProgram { b_expr: tr_r(root) };
    return Box::new(b_program);
}

fn tr_r(root: Box<dyn Any>) -> Box<dyn Any> {
    let actual_id = (&*root).type_id();
    if actual_id == TypeId::of::<ZExpr>() {
        let mut bash_expr = BExpr { childs: vec![] };
        let z_expr = root.downcast::<ZExpr>().unwrap();
        for child in z_expr.childs {
            bash_expr.childs.push(tr_r(child));
        }

        return Box::new(bash_expr);
    } else if actual_id == TypeId::of::<ZFunction_call>() {
        let mut z_func_call = root.downcast::<ZFunction_call>().unwrap();
        return Box::new(BEcho {
            to_echo: tr_r(z_func_call.parameters.remove(0)),
        });
    } else if actual_id == TypeId::of::<ZAssignment>() {
        let z_assignment = root.downcast::<ZAssignment>().unwrap();
        return Box::new(BAssignment {
            iden: z_assignment.iden,
            content: tr_r(z_assignment.content),
        });
    } else if actual_id == TypeId::of::<ZIden>() {
        let z_iden = root.downcast::<ZIden>().unwrap();
        return Box::new(BIden { name: z_iden.name });
    } else if actual_id == TypeId::of::<ZString>() {
        let z_string = root.downcast::<ZString>().unwrap();
        return Box::new(BString { val: z_string.val });
    } else if actual_id == TypeId::of::<ZNumber>() {
        let z_number = root.downcast::<ZNumber>().unwrap();
        return Box::new(BNumber { val: z_number.val });
    }

    panic!("Don't know what this is");
}
