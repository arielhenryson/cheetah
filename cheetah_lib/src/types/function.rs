use super::expression::Expression;

#[derive(Debug, Clone)]
pub enum Function {
    // A regular function
    RegularFunc(RegularFunction),
}

// Represents a regular javascript function in memory
#[derive(Debug, Clone)]
pub struct RegularFunction {
    // The function name
    pub name: Option<String>,

    // The function arguments
    pub args: Vec<Expression>,

    // This function's expression
    pub expression: Expression
}

impl RegularFunction {
    /// Make a new regular function
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(name: Option<String>, args: Vec<Expression>, expression: Expression) -> Self {
        Self {
            name,
            args,
            expression
        }
    }
}