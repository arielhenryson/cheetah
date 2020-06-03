use super::super::parser::{ Parser, ParseResult };
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::constant::Const;

impl Parser {
    pub fn number_expression(&mut self, value: String) -> ParseResult {
        Ok(
            Expression::new(
                ExpressionDef::Const(
                    Const::Int(
                        value.parse::<i32>().unwrap()
                    )
                )
            )
        )
    }
}