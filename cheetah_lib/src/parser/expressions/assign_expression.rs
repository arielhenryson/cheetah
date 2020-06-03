use super::super::parser::{ Parser, ParseResult };
use crate::types::expression::{ Expression, ExpressionDef };

impl Parser {
    pub fn assign_expression(&mut self, expression: Expression) -> ParseResult {
        self.pos += 1;
        let next = self.parse()?;

        Ok(
            Expression::new(
                ExpressionDef::Assign(
                    Box::new(expression),
                    Box::new(next)
                )
            )
        )
    }
}