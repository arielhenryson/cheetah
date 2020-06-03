use super::parser::Parser;
use super::error::ParseError;
use crate::types::token_data::TokenData;


impl Parser {
    pub fn get_token(&mut self, pos: usize) -> Result<TokenData, ParseError> {
        if pos < self.tokens.len() {
            return Ok(
                self.tokens.get(pos).expect("failed getting token").clone()
            );
        }

        Err(ParseError::EndOfTokens)
    }
}