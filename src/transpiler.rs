use std::{
    any::{Any, TypeId},
    vec,
};

use crate::{
    bash_nodes::{
        bassignment::BAssignment, becho::BEcho, bmath_expr::BMathExpr, bprogram::BProgram,
    },
    nodes::{
        bin_op::BinOp, function_call::FunctionCall, identifier::Iden, number::Number,
        program::Expr, variable_statement::Assignment,
    },
};

pub fn transpile(root: Box<Expr>) -> Box<BProgram> {
    let b_program = BProgram {
        b_expr: tr_r(root, false),
    };
    return Box::new(b_program);
}

fn tr_r(root: Box<dyn Any>, last_math: bool) -> Box<dyn Any> {
    let actual_id = (&*root).type_id();
    if actual_id == TypeId::of::<Expr>() {
        let mut z_expr = root.downcast::<Expr>().unwrap();
        z_expr.childs = z_expr.childs.into_iter().map(|c| tr_r(c, false)).collect();
        return z_expr;
    } else if actual_id == TypeId::of::<FunctionCall>() {
        let z_func_call = root.downcast::<FunctionCall>().unwrap();
        let mut params = vec![];
        for param in z_func_call.parameters {
            params.push(tr_r(param, false));
        }
        return Box::new(BEcho { to_echo: params });
    } else if actual_id == TypeId::of::<Assignment>() {
        let z_assignment = root.downcast::<Assignment>().unwrap();
        return Box::new(BAssignment {
            iden: z_assignment.iden,
            content: tr_r(z_assignment.content, false),
        });
    } else if actual_id == TypeId::of::<BinOp>() {
        let mut z_bin_op = root.downcast::<BinOp>().unwrap();
        z_bin_op.left = tr_r(z_bin_op.left, true);
        z_bin_op.right = tr_r(z_bin_op.right, true);
        if last_math {
            return z_bin_op;
        }

        return Box::new(BMathExpr { content: z_bin_op });
    } else if actual_id == TypeId::of::<Iden>() {
        return root;
    } else if actual_id == TypeId::of::<String>() {
        return root;
    } else if actual_id == TypeId::of::<Number>() {
        return root;
    }

    panic!("Don't know what this is");
}
