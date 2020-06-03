use super::parser::{ Parser, ParseResult };
use super::error::ParseError;

impl Parser {
    pub fn skip_expression(&mut self) -> ParseResult {
        // if its not the end of the tokens move next
        if self.pos < self.tokens.len() {
            return self.parse();
        }

        Err(ParseError::EndOfTokens)
    }
}