use std::{
    any::{Any, TypeId},
    vec,
};

use crate::{
    bash_nodes::{
        bassignment::BAssignment, bbin_op::BBinOp, becho::BEcho, bexpr::BExpr, biden::BIden,
        bmath_expr::BMathExpr, bnumber::BNumber, bprogram::BProgram, bstring::BString,
    },
    zl_nodes::{
        zassignment::ZAssignment, zbin_op::ZBinOp, zexpr::ZExpr, zfunction_call::ZFunctionCall,
        ziden::ZIden, znumber::ZNumber, zstring::ZString,
    },
};

pub fn transpile(root: Box<ZExpr>) -> Box<BProgram> {
    let b_program = BProgram {
        b_expr: tr_r(root, false),
    };
    return Box::new(b_program);
}

fn tr_r(root: Box<dyn Any>, last_math: bool) -> Box<dyn Any> {
    let actual_id = (&*root).type_id();
    if actual_id == TypeId::of::<ZExpr>() {
        let mut bash_expr = BExpr { childs: vec![] };
        let z_expr = root.downcast::<ZExpr>().unwrap();
        for child in z_expr.childs {
            bash_expr.childs.push(tr_r(child, false));
        }

        return Box::new(bash_expr);
    } else if actual_id == TypeId::of::<ZFunctionCall>() {
        let z_func_call = root.downcast::<ZFunctionCall>().unwrap();
        let mut params = vec![];
        for param in z_func_call.parameters {
            params.push(tr_r(param, false));
        }
        return Box::new(BEcho { to_echo: params });
    } else if actual_id == TypeId::of::<ZAssignment>() {
        let z_assignment = root.downcast::<ZAssignment>().unwrap();
        return Box::new(BAssignment {
            iden: z_assignment.iden,
            content: tr_r(z_assignment.content, false),
        });
    } else if actual_id == TypeId::of::<ZBinOp>() {
        let z_bin_op = root.downcast::<ZBinOp>().unwrap();
        let bin_op = Box::new(BBinOp {
            op: z_bin_op.op,
            parenthese: z_bin_op.parenthese,
            left: tr_r(z_bin_op.left, true),
            right: tr_r(z_bin_op.right, true),
        });
        if last_math {
            return bin_op;
        }

        return Box::new(BMathExpr { content: bin_op });
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
