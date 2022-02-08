use std::any::{Any, TypeId};

use crate::{
    bash_nodes::bvariable_statement::BVariableStatement,
    nodes::{
        expression_statement::NExpressionStatement, identifier::NIdentifier, number::NNumber,
        program::NProgram,
    },
};

pub fn generate_code(root: Box<dyn Any>) -> Box<String> {
    let mut str = String::new();
    let actual_id = (&*root).type_id();
    if actual_id == TypeId::of::<NProgram>() {
        let bash_program = root.downcast::<NProgram>().unwrap();
        str += "#!/bin/bash\n";
        for child in bash_program.childs {
            str += &(generate_code(child).to_string() + "\n")
        }
    } else if actual_id == TypeId::of::<BVariableStatement>() {
        let variable_statement = root.downcast::<BVariableStatement>().unwrap();
        str += &(generate_code(variable_statement.identifier).to_string()
            + "="
            + &generate_code(variable_statement.expression));
    } else if actual_id == TypeId::of::<NExpressionStatement>() {
        let expression_statement = root.downcast::<NExpressionStatement>().unwrap();
        str += generate_code(expression_statement.content).as_str();
    } else if actual_id == TypeId::of::<NIdentifier>() {
        let identifier = root.downcast::<NIdentifier>().unwrap();
        str += &("$".to_string() + &identifier.name);
    } else if actual_id == TypeId::of::<NNumber>() {
        let number = root.downcast::<NNumber>().unwrap();
        str += &number.val.to_string();
    }

    return Box::new(str);
}
