// use std::any::{Any, TypeId};

// use crate::{
//     bash_nodes::bvariable_statement::BVariableStatement,
//     nodes::{
//         expression_statement::NExpressionStatement, identifier::NIdentifier, number::NNumber,
//         program::NProgram, variable_statement::NVariableStatement,
//     },
// };

// pub fn transpile(mut root: Box<NProgram>) -> Box<NProgram> {
//     root.childs = root.childs.into_iter().map(|c| tr_r(c)).collect();
//     return root;
// }

// fn tr_r(root: Box<dyn Any>) -> Box<dyn Any> {
//     let actual_id = (&*root).type_id();
//     if actual_id == TypeId::of::<NVariableStatement>() {
//         let variable_statement = root.downcast::<NVariableStatement>().unwrap();
//         return Box::new(BVariableStatement {
//             identifier: tr_r(variable_statement.identifier),
//             expression: tr_r(variable_statement.expression),
//         });
//     } else if actual_id == TypeId::of::<NExpressionStatement>() {
//         let mut expression_statement = root.downcast::<NExpressionStatement>().unwrap();
//         expression_statement.content = tr_r(expression_statement.content);
//         return expression_statement;
//     } else if actual_id == TypeId::of::<NIdentifier>() {
//         return root;
//     } else if actual_id == TypeId::of::<NNumber>() {
//         return root;
//     }

//     panic!("Don't know what this is");
// }
