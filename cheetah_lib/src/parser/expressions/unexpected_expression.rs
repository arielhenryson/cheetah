use super::super::parser::{ Parser, ParseResult };
use super::super::error::ParseError;
use crate::types::token_data::TokenData;

impl Parser {
    pub fn unexpected_expression(&mut self, token: TokenData) -> ParseResult {
        let (line, column) = self.error_handler.throw(
            token.clone()
        );

        return Err(
            ParseError::UnexpectedToken(token.value, line, column)
        );
    }
}