use super::super::parser::{ Parser, ParseResult };
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::tokens::Token;

impl Parser {
    pub fn array_expression(&mut self) -> ParseResult {
        let mut items = Vec::new();

        loop {
            let mut next = self.get_token(self.pos)?;

            // check if end of array
            if next.kind == Token::BracketClose {
                self.pos += 1;
                break;
            }

            // push the item to the array
            let res = self.parse()?;
            items.push(res);

            // after array item we check if we have comma
            // and if so we skip to the next item
            next = self.get_token(self.pos)?;
            if next.kind == Token::Comma {
                self.pos += 1;
            }
        }

        Ok(
            Expression::new(
                ExpressionDef::ArrayDecl(items)
            )
        )
    }
}