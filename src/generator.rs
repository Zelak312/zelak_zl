use crate::{
    ast::{
        node_box::NodeBox,
        node_kind::NodeKind,
        nodes::{
            array::NArray, boolean::NBoolean, call_statement::NCallStatement,
            condition::NCondition, condition_statement::NConditionStatement,
            expression_statement::NExpressionStatement, function_definition::NFunctionDefinition,
            function_return::NFunctionReturn, identifier::NIdentifier, if_statement::NIfStatement,
            math_statement::NMathStatement, number::NNumber,
            parenthese_statement::NParentheseStatement, program::NProgram, string::NString,
            string_concat::NStringConcat, variable_statement::NVariableStatement,
        },
    },
    bash_nodes::{
        bmath_expression::BMathExpression, bstring_concat_expression::BStringConcatExpression,
    },
};

pub fn generate_code(node: Box<NodeBox>, tab: usize, in_concat: bool) -> Box<String> {
    let str = match node._type {
        NodeKind::Program => {
            let data = node.content.downcast::<NProgram>().unwrap();
            let mut start = "#!/bin/bash\n".to_string();
            for child in data.childs {
                start += &format!("{}\n", generate_code(child, 0, false));
            }
            start
        }
        NodeKind::FunctionDefinition => {
            let data = node.content.downcast::<NFunctionDefinition>().unwrap();
            let identifier = data.identifier.content.downcast::<NIdentifier>().unwrap();
            let mut start = format!("function {} {{\n", identifier.name);
            let mut counter = 1;
            for argument in data.arguments {
                let identifier = argument.content.downcast::<NIdentifier>().unwrap();
                start += &format!(
                    "{}local {}=${}\n",
                    "\t".repeat(tab + 1),
                    identifier.name,
                    counter
                );
                counter += 1;
            }
            for expression in data.expressions {
                start += &format!("{}\n", generate_code(expression, tab + 1, false));
            }
            format!("{}{}}}", start, "\t".repeat(tab))
        }
        NodeKind::FunctionReturn => {
            let data = node.content.downcast::<NFunctionReturn>().unwrap();
            format!("return {}", generate_code(data.content, tab, false))
        }
        NodeKind::IfStatement => {
            let data = node.content.downcast::<NIfStatement>().unwrap();
            let mut start = format!(
                "if {}\n{}then\n",
                generate_code(data.condition, tab, false),
                "\t".repeat(tab)
            );
            for expression in data.expressions {
                start += &format!("{}\n", generate_code(expression, tab + 1, false));
            }
            format!("{}{}fi", start, "\t".repeat(tab))
        }
        NodeKind::ForStatement => "Not implemented for loops".to_string(),
        NodeKind::VariableStatement => {
            let data = node.content.downcast::<NVariableStatement>().unwrap();
            let identifier = data.identifier.content.downcast::<NIdentifier>().unwrap();
            let mut d = String::new();
            if data.expression._type == NodeKind::CallStatement {
                d = format!(
                    "{}\n{}{}=$?",
                    generate_code(data.expression, tab, false),
                    "\t".repeat(tab),
                    identifier.name
                );
            } else {
                d = format!(
                    "{}={}",
                    identifier.name,
                    generate_code(data.expression, tab, false)
                );
            }
            d
        }
        NodeKind::ExpressionStatement => {
            let data = node.content.downcast::<NExpressionStatement>().unwrap();
            format!(
                "{}{}",
                "\t".repeat(tab),
                generate_code(data.content, tab, false)
            )
        }
        NodeKind::CallStatement => {
            let data = node.content.downcast::<NCallStatement>().unwrap();
            let identifier = data.identifier.content.downcast::<NIdentifier>().unwrap();
            let mut start = match identifier.name.as_str() {
                "print" => "echo".to_string(),
                _ => identifier.name,
            } + " ";
            let mut first = true;
            for parameter in data.parameters {
                if first {
                    start += &generate_code(parameter, tab, in_concat);
                    first = false;
                } else {
                    start += &format!(" {}", generate_code(parameter, tab, in_concat));
                }
            }
            start
        }
        NodeKind::ConditionStatement => {
            let data = node.content.downcast::<NConditionStatement>().unwrap();
            format!(
                "{} {} {}",
                generate_code(data.left, tab, in_concat),
                data.operator,
                generate_code(data.right, tab, in_concat)
            )
        }
        NodeKind::Condition => {
            let data = node.content.downcast::<NCondition>().unwrap();
            let op = match data.operator.as_str() {
                "==" => "=",
                "!=" => "!=",
                ">" => "-gt",
                "<" => "-lt",
                _ => panic!("Don't know this operator"),
            };

            format!(
                "[ {} {} {} ]",
                generate_code(data.left, tab, in_concat),
                op,
                generate_code(data.right, tab, in_concat)
            )
        }
        NodeKind::ParentheseStatement => {
            let data = node.content.downcast::<NParentheseStatement>().unwrap();
            format!("({})", generate_code(data.content, tab, in_concat))
        }
        NodeKind::Array => {
            let data = node.content.downcast::<NArray>().unwrap();
            let mut start = "(".to_string();
            let mut first = true;
            for parameter in data.items {
                if first {
                    start += &generate_code(parameter, tab, in_concat);
                    first = false;
                } else {
                    start += &format!(" {}", generate_code(parameter, tab, false));
                }
            }
            format!("{})", start)
        }
        NodeKind::Identifier => {
            let data = node.content.downcast::<NIdentifier>().unwrap();
            format!("${}", data.name)
        }
        NodeKind::String => {
            let data = node.content.downcast::<NString>().unwrap();
            let d;
            if in_concat {
                d = format!("{}", data.val);
            } else {
                d = format!("\"{}\"", data.val)
            }
            d
        }
        NodeKind::Number => {
            let data = node.content.downcast::<NNumber>().unwrap();
            format!("{}", data.val)
        }
        NodeKind::Boolean => {
            let data = node.content.downcast::<NBoolean>().unwrap();
            format!("{}", data.val)
        }
        NodeKind::MathStatement => {
            let data = node.content.downcast::<NMathStatement>().unwrap();
            format!(
                "{} {} {}",
                generate_code(data.left, tab, in_concat),
                data.operator,
                generate_code(data.right, tab, in_concat)
            )
        }
        NodeKind::StringConcat => {
            let data = node.content.downcast::<NStringConcat>().unwrap();
            format!(
                "{}{}",
                generate_code(data.left, tab, true),
                generate_code(data.right, tab, true)
            )
        }
        NodeKind::BMathExpression => {
            let data = node.content.downcast::<BMathExpression>().unwrap();
            format!("$(({}))", generate_code(data.content, tab, in_concat))
        }
        NodeKind::BStringConcatExpression => {
            let data = node.content.downcast::<BStringConcatExpression>().unwrap();
            format!("\"{}\"", generate_code(data.content, tab, false))
        }
    };

    return Box::new(str);
}
