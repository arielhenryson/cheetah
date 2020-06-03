use super::constant::Const;

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub def: ExpressionDef
}

pub type HeapExpr = Box<Expression>;

impl Expression {
    pub fn new(def: ExpressionDef) -> Self {
        Self { def }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionDef {
    // Run several expressions from top-to-bottom
    Block(Vec<Expression>),

    // Make a constant value
    Const(Const),

    // Run a operation between 2 expressions
    BinOp(String, HeapExpr, HeapExpr),

    // Return the expression from a function
    Return(
        Option<HeapExpr>
    ),

    // Create a function with the given name, and expression
    FunctionDecl(Option<String>, Vec<Expression>, HeapExpr),

    // Call a function
    Call(
        // the target name
        String,
        // the arguments
        Vec<Expression>,
        DebugInfo
    ),

    // Create a variable with the given name
    VariableDecl(String, HeapExpr),

    // Array
    ArrayDecl(Vec<Expression>),

    // get member of array
    Member(HeapExpr, HeapExpr),

    // represent variable name or func name
    Identifier(String, DebugInfo),

    // Assign an expression to a value
    Assign(HeapExpr, HeapExpr),

    // if expression
    If(HeapExpr, HeapExpr, Option<HeapExpr>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct DebugInfo {
    pub start: usize,
}