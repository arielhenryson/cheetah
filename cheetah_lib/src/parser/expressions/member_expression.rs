use super::super::parser::{ Parser, ParseResult } ;
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::tokens::Token;

impl Parser {
    pub fn member_expression(&mut self, object: Expression) -> ParseResult {
        self.pos += 1;

        let value = self.parse()?;

        self.expect(self.pos, Token::BracketClose);

        self.pos += 1;

        Ok(
            Expression::new(
                ExpressionDef::Member(
                    Box::new(object),
                    Box::new(value)
                )
            )
        )
    }
}