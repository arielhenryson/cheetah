use crate::parser::parser::{ Parser, ParseResult };
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::tokens::Token;
use super::super::super::error::ParseError;

impl Parser {
    pub fn function_expression(&mut self) -> ParseResult {
        // function [identifier] () { block }
        let token = self.get_token(self.pos)?;

        let name = self.get_func_name(
            token.clone()
        );

        let args = self.get_parameters()?;

        // check if the next
        // token is BraceOpen and if not stop the engine
        self.check_for_block_open();

        let block = self.parse()?;

        return match name {
            FuncName::UnexpectedToken => {
                let (line, column) = self.error_handler.throw(token.clone());

                Err(
                    ParseError::UnexpectedToken(token.value, line, column)
                )
            },
            FuncName::Name(name) => {
                Ok(
                    Expression::new(
                        ExpressionDef::FunctionDecl(
                            Some(name),
                            args,
                            Box::new(block)
                        )
                    )
                )
            },
            FuncName::ParenOpen => {
                Ok(
                    Expression::new(
                        ExpressionDef::FunctionDecl(
                            None,
                            args,
                            Box::new(block)
                        )
                    )
                )
            }
        }
    }

    fn check_for_block_open(&mut self) {
        self.expect(self.pos, Token::BraceOpen);
    }
}

#[derive(Debug)]
pub enum FuncName {
    ParenOpen,

    Name(String),

    UnexpectedToken
}