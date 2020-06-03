use super::super::parser::{ Parser, ParseResult } ;
use crate::types::expression::{ Expression, ExpressionDef };

impl Parser {
    pub fn return_expression(&mut self) -> ParseResult {
        if self.pos == self.tokens.len() {
            return Ok(
                Expression::new(
                    ExpressionDef::Return(
                        None
                    )
                )
            );
        }

        let mut box_expr = None;

        // if the next value is not semicolon
        // we want to return it
        let next_token = self.get_token(self.pos)?;

        if next_token.value != String::from(";") {
            box_expr = Some(
                Box::new(
                    self.parse()?
                )
            );
        }

        Ok(
            Expression::new(
                ExpressionDef::Return(box_expr)
            )
        )
    }
}