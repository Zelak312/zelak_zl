use std::any::{Any, TypeId};

use crate::{
    bash_nodes::bvariable_statement::BVariableStatement,
    nodes::{
        expression_statement::NExpressionStatement, identifier::NIdentifier, number::NNumber,
        program::NProgram, variable_statement::NVariableStatement,
    },
};

pub fn transpile(mut root: Box<NProgram>) -> Box<NProgram> {
    root.childs = root.childs.into_iter().map(|c| tr_r(c, false)).collect();
    return root;
}

fn tr_r(root: Box<dyn Any>, last_math: bool) -> Box<dyn Any> {
    let actual_id = (&*root).type_id();
    if actual_id == TypeId::of::<NVariableStatement>() {
        let variable_statement = root.downcast::<NVariableStatement>().unwrap();
        return Box::new(BVariableStatement {
            iden: tr_r(variable_statement.identifier, false),
            expression: tr_r(variable_statement.expression, false),
        });
    } else if actual_id == TypeId::of::<NExpressionStatement>() {
        return root;
    } else if actual_id == TypeId::of::<NIdentifier>() {
        return root;
    } else if actual_id == TypeId::of::<NNumber>() {
        return root;
    }

    panic!("Don't know what this is");
}
