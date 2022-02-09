#[derive(PartialEq, Clone)]
pub enum NodeKind {
    Program,
    IfStatement,
    ForStatement,
    VariableStatement,
    ExpressionStatement,
    CallStatement,
    ConditionStatement,
    Condition,
    MathStatement,
    ParentheseStatement,
    Identifier,
    String,
    Number,
}
