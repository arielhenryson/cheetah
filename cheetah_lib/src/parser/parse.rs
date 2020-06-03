use super::parser::{ Parser, ParseResult };
use super::error::ParseError;
use crate::types::tokens::Token;
use crate::types::expression::DebugInfo;

impl Parser {
    pub fn parse(&mut self) -> ParseResult {
        if self.pos == self.tokens.len() {
            return Err(ParseError::EndOfTokens);
        }

        let token = self.get_token(self.pos)?;

        self.pos += 1;

        let expression = match token.kind {
            Token::Semicolon        => self.skip_expression()?,

            Token::KeywordReturn    => self.return_expression()?,

            Token::Number           => self.number_expression(token.value)?,

            Token::DeclarationFunction
                                    => self.function_expression()?,

            Token::Identifier       => self.identifier_expression(token.value, DebugInfo {
                start: token.position.start
            })?,

            Token::BraceOpen        => self.block_expression()?,

            Token::DeclarationVariable
                                    => self.variable_expression()?,

            Token::BracketOpen
                                    => self.array_expression()?,

            Token::KeywordIf        => self.if_expression()?,

            Token::NewLine          => self.skip_expression()?,

            _                       => self.unexpected_expression(token)?
        };

        let end_of_token = self.pos >= self.tokens.len();

        if end_of_token {
            return Ok(expression);
        }

        self.parse_next(expression)
    }
}