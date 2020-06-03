use super::super::parser::{ Parser, ParseResult };
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::expression::DebugInfo;

impl Parser {
    pub fn identifier_expression(&mut self, name: String, debug_info:  DebugInfo) -> ParseResult {
        Ok(
            Expression::new(
                ExpressionDef::Identifier(
                    name,
                    debug_info
                )
            )
        )
    }
}
