#[derive(PartialEq, Clone)]
pub enum NodeKind {
    Program,
    VariableStatement,
    ExpressionStatement,
    CallStatement,
    MathStatement,
    Identifier,
    String,
    Number,
}
