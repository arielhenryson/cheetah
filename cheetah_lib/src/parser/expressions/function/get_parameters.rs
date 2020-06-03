use crate::parser::parser::Parser;
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::tokens::Token;
use super::super::super::error::ParseError;
use crate::types::expression::DebugInfo;

impl Parser {
    pub fn get_parameters(&mut self) -> Result<Vec<Expression>, ParseError> {
        self.pos += 1; // skip (

        let mut params = Vec::new();

        let mut token = self.get_token(self.pos)?;

        while token.kind != Token::ParenClose {
            match token.kind {
                Token::Identifier => params.push(
                    Expression::new(
                        ExpressionDef::Identifier(token.value.clone(), DebugInfo {
                            start: token.position.start
                        })
                    )
                ),
                _ => {
                    return Err(
                        ParseError::UnexpectedToken(
                            String::from(""),
                            0,
                            0
                        )
                    )
                }
            }

            self.pos += 1;

            let tk = self.get_token(self.pos)?;
            if tk.kind == Token::Comma {
                self.pos += 1;
            }

            token = self.get_token(self.pos)?;
        }

        self.pos += 1;

        Ok(
            params
        )
    }
}