#[derive(PartialEq, Clone)]
pub enum NodeKind {
    Program,
    IfStatement,
    ConditionStatement,
    Condition,
    VariableStatement,
    ExpressionStatement,
    CallStatement,
    MathStatement,
    Identifier,
    String,
    Number,
}
