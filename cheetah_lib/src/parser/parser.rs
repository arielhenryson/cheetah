use crate::types::token_data::TokenData;
use super::error::ParseError;
use crate::types::expression::{ Expression, ExpressionDef };
use crate::error_handler::error_handler::ErrorHandler;

#[derive(Debug)]
pub struct Parser {
    // The tokens being input
    pub tokens: Vec<TokenData>,

    // The current position within the tokens
    pub pos: usize,

    // handle error
    pub error_handler: ErrorHandler
}

type AstResult = Result<Expression, String>;
pub type ParseResult = Result<Expression, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<TokenData>, error_handler: ErrorHandler) -> Self {
        Self {
            tokens,
            error_handler,
            pos: 0
        }
    }

    pub fn start(&mut self) -> AstResult {
        let mut ast = Vec::new();

        while self.pos < self.tokens.len() {
            let expression = self.parse();
            match expression {
                Ok(node) => {
                    ast.push(node);
                },
                Err(e) => {
                    if e != ParseError::EndOfTokens {
                        return Err(
                            String::from("")
                        )
                    }
                },
            }
        }

        Ok(
            Expression::new(
                ExpressionDef::Block(ast)
            )
        )
    }
}
