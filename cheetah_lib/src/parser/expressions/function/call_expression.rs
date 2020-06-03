use super::super::super::parser::{ Parser, ParseResult };
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::expression::DebugInfo;

impl Parser {
    pub fn call_expression(&mut self, name: String, debug_info:  DebugInfo) -> ParseResult {
        let args = self.get_arguments()?;


        Ok(
            Expression::new(
                ExpressionDef::Call(
                    name,
                    args,
                    debug_info
                )
            )
        )
    }
}