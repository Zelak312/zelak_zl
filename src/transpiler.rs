use crate::{
    ast::{
        node_box::NodeBox,
        node_kind::NodeKind,
        nodes::{
            array::NArray, call_statement::NCallStatement, condition::NCondition,
            condition_statement::NConditionStatement, expression_statement::NExpressionStatement,
            for_statement::NForStatement, function_definition::NFunctionDefinition,
            function_return::NFunctionReturn, if_statement::NIfStatement,
            math_statement::NMathStatement, parenthese_statement::NParentheseStatement,
            program::NProgram, string_concat::NStringConcat,
            variable_statement::NVariableStatement,
        },
    },
    bash_nodes::{
        bmath_expression::BMathExpression, bstring_concat_expression::BStringConcatExpression,
    },
};

pub fn transpile(root: Box<NodeBox>) -> Box<NodeBox> {
    let mut program = root.content.downcast::<NProgram>().unwrap();
    program.childs = program
        .childs
        .into_iter()
        .map(|c| tr_r(c, false, false))
        .collect();
    return NodeBox::new_box(program, NodeKind::Program);
}

fn tr_r(mut node: Box<NodeBox>, in_math: bool, in_string_concat: bool) -> Box<NodeBox> {
    match node._type {
        NodeKind::FunctionDefinition => {
            let mut data = node.content.downcast::<NFunctionDefinition>().unwrap();
            data.identifier = tr_r(data.identifier, false, false);
            data.arguments = data
                .arguments
                .into_iter()
                .map(|a| tr_r(a, false, false))
                .collect();
            data.expressions = data
                .expressions
                .into_iter()
                .map(|e| tr_r(e, false, false))
                .collect();
            return NodeBox::new_box(data, NodeKind::FunctionDefinition);
        }
        NodeKind::FunctionReturn => {
            let mut data = node.content.downcast::<NFunctionReturn>().unwrap();
            data.content = tr_r(data.content, false, false);
            return NodeBox::new_box(data, NodeKind::FunctionReturn);
        }
        NodeKind::IfStatement => {
            let mut data = node.content.downcast::<NIfStatement>().unwrap();
            data.condition = tr_r(data.condition, false, false);
            data.expressions = data
                .expressions
                .into_iter()
                .map(|e| tr_r(e, false, false))
                .collect();
            return NodeBox::new_box(data, NodeKind::IfStatement);
        }
        NodeKind::ForStatement => {
            let mut data = node.content.downcast::<NForStatement>().unwrap();
            data.start = data.start.and_then(|s| Some(tr_r(s, false, false)));
            data.condition = tr_r(data.condition, false, false);
            data.step = data.step.and_then(|s| Some(tr_r(s, false, false)));
            data.expressions = data
                .expressions
                .into_iter()
                .map(|e| tr_r(e, false, false))
                .collect();
            return NodeBox::new_box(data, NodeKind::ForStatement);
        }
        NodeKind::VariableStatement => {
            let mut data = node.content.downcast::<NVariableStatement>().unwrap();
            data.declare_type = None;
            data.identifier = tr_r(data.identifier, false, false);
            data.expression = tr_r(data.expression, false, false);
            return NodeBox::new_box(data, NodeKind::VariableStatement);
        }
        NodeKind::ExpressionStatement => {
            let mut data = node.content.downcast::<NExpressionStatement>().unwrap();
            data.content = tr_r(data.content, false, false);
            return NodeBox::new_box(data, NodeKind::ExpressionStatement);
        }
        NodeKind::CallStatement => {
            let mut data = node.content.downcast::<NCallStatement>().unwrap();
            data.identifier = tr_r(data.identifier, in_math, in_string_concat);
            data.parameters = data
                .parameters
                .into_iter()
                .map(|p| tr_r(p, false, false))
                .collect();
            return NodeBox::new_box(data, NodeKind::CallStatement);
        }
        NodeKind::ConditionStatement => {
            let mut data = node.content.downcast::<NConditionStatement>().unwrap();
            data.left = tr_r(data.left, in_math, in_string_concat);
            data.right = tr_r(data.right, in_math, in_string_concat);
            return NodeBox::new_box(data, NodeKind::ConditionStatement);
        }
        NodeKind::Condition => {
            let mut data = node.content.downcast::<NCondition>().unwrap();
            data.left = tr_r(data.left, in_math, in_string_concat);
            data.right = tr_r(data.right, in_math, in_string_concat);
            return NodeBox::new_box(data, NodeKind::Condition);
        }
        NodeKind::ParentheseStatement => {
            let mut data = node.content.downcast::<NParentheseStatement>().unwrap();
            data.content = tr_r(data.content, in_math, in_string_concat);
            return NodeBox::new_box(data, NodeKind::ParentheseStatement);
        }
        NodeKind::Array => {
            let mut data = node.content.downcast::<NArray>().unwrap();
            data.items = data
                .items
                .into_iter()
                .map(|i| tr_r(i, false, false))
                .collect();
            return NodeBox::new_box(data, NodeKind::Array);
        }
        NodeKind::Identifier | NodeKind::String | NodeKind::Number | NodeKind::Boolean => node,
        NodeKind::MathStatement => {
            let mut data = node.content.downcast::<NMathStatement>().unwrap();
            data.left = tr_r(data.left, true, in_string_concat);
            data.right = tr_r(data.right, true, in_string_concat);
            let node_box = NodeBox::new_box(data, NodeKind::MathStatement);
            if !in_math {
                return NodeBox::new_box(
                    Box::new(BMathExpression { content: node_box }),
                    NodeKind::BMathExpression,
                );
            }

            return node_box;
        }
        NodeKind::StringConcat => {
            let mut data = node.content.downcast::<NStringConcat>().unwrap();
            data.left = tr_r(data.left, in_math, true);
            data.right = tr_r(data.right, in_math, true);
            let node_box = NodeBox::new_box(data, NodeKind::StringConcat);
            if !in_string_concat {
                return NodeBox::new_box(
                    Box::new(BStringConcatExpression { content: node_box }),
                    NodeKind::BStringConcatExpression,
                );
            }

            return node_box;
        }
        _ => panic!("Don't know what this is"),
    }

    // if actual_id == TypeId::of::<NVariableStatement>() {
    //     let variable_statement = root.downcast::<NVariableStatement>().unwrap();
    //     return Box::new(BVariableStatement {
    //         identifier: tr_r(variable_statement.identifier),
    //         expression: tr_r(variable_statement.expression),
    //     });
    // } else if actual_id == TypeId::of::<NExpressionStatement>() {
    //     let mut expression_statement = root.downcast::<NExpressionStatement>().unwrap();
    //     expression_statement.content = tr_r(expression_statement.content);
    //     return expression_statement;
    // } else if actual_id == TypeId::of::<NIdentifier>() {
    //     return root;
    // } else if actual_id == TypeId::of::<NNumber>() {
    //     return root;
    // }
}
