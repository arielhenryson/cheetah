use crate::parser::parser::Parser;
use crate::types::tokens::Token;
use crate::types::token_data::TokenData;
use super::function_expression::FuncName;

impl Parser {
    pub fn get_func_name(&mut self, token: TokenData) -> FuncName {
        match token.kind {
            Token::Identifier => {
                self.pos += 1;
                FuncName::Name(token.value)
            }
            Token::ParenOpen => FuncName::ParenOpen,
            _ => FuncName::UnexpectedToken
        }
    }
}
