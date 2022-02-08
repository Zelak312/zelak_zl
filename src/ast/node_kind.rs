#[derive(PartialEq, Clone)]
pub enum NodeKind {
    Program,
    IfStatement,
    ConditionStatement,
    Condition,
    VariableStatement,
    ExpressionStatement,
    CallStatement,
    ParentheseStatement,
    MathStatement,
    Identifier,
    String,
    Number,
}
