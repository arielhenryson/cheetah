use super::parser::Parser;
use crate::types::tokens::Token;

impl Parser {
    pub fn expect(&mut self, pos: usize, token_type: Token) {
        let token = self.get_token(pos).unwrap();


        if token.kind != token_type {
            self.error_handler.throw(token.clone());
            std::process::exit(1)
        }
    }
}