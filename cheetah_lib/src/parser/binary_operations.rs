use super::parser::{ Parser, ParseResult };
use crate::types::expression::{ Expression, ExpressionDef };

impl Parser {
    pub fn binary_operations(&mut self, operation: String, expression: Expression) -> ParseResult {
        self.pos += 1;

        let next_expression = self.parse()?;

        Ok(
            Expression::new(
                ExpressionDef::BinOp(
                    String::from(operation),
                    Box::new(expression),
                    Box::new(next_expression)
                )
            )
        )
    }
}