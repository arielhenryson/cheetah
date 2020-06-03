use super::super::parser::{ Parser, ParseResult };
use crate::types::tokens::Token;
use crate::types::expression::{ Expression, ExpressionDef };

impl Parser {
    pub fn block_expression(&mut self) -> ParseResult {
        let mut exprs = Vec::new();

        loop {
            let token = self.get_token(self.pos)?;

            if token.kind == Token::NewLine {
                self.pos += 1;
                continue;
            }

            if token.kind == Token::BraceClose {
                break;
            }

            exprs.push(
                self.parse()?
            );
        }

        self.pos += 1;

        Ok(
            Expression::new(
                ExpressionDef::Block(
                    exprs
                )
            )
        )
    }
}