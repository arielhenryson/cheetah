use super::super::parser::{ Parser, ParseResult } ;
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::tokens::Token;

impl Parser {
    pub fn if_expression(&mut self) -> ParseResult {
        self.expect(self.pos, Token::ParenOpen);

        self.pos += 1;

        let condition = self.parse()?;

        self.expect(self.pos, Token::ParenClose);

        self.pos += 1;

        let expression = self.parse()?;

        let mut else_block = None;

        let next = self.get_token(self.pos);

        // if there is a 'else' block
        if next.is_ok() && next.unwrap().kind == Token::KeywordElse {
            self.pos += 1;

            else_block = Some(
                Box::new(
                    self.parse()?
                )
            );
        }

        Ok(
            Expression::new(
                ExpressionDef::If(
                    Box::new(condition),
                    Box::new(expression),
                    else_block
                )
            )
        )
    }
}