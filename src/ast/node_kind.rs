#[derive(PartialEq, Clone)]
pub enum NodeKind {
    Program,
    IfStatement,
    Condition,
    VariableStatement,
    ExpressionStatement,
    CallStatement,
    MathStatement,
    Identifier,
    String,
    Number,
}
