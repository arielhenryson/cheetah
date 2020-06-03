use crate::parser::parser::Parser;
use crate::types::tokens::Token;
use crate::types::expression::{ Expression };
use crate::parser::error::ParseError;
use crate::types::token_data::TokenData;

type VecOfExpression = Vec<Expression>;
type ArgsResult = Result<VecOfExpression, ParseError>;

impl Parser {
    pub fn get_arguments(&mut self) -> ArgsResult  {
        let mut args = Vec::new();

        self.pos += 1;

        // Handle situation where there is no arguments
        let token = self.get_token(self.pos)?;
        let no_params = token.kind == Token::ParenClose;
        if no_params {
            self.pos += 1;

            return Ok(args);
        }

        let mut expect_comma_or_end = false;

        // Loop through all arguments
        loop {
            let token = self.get_token(self.pos)?;

            if !expect_comma_or_end {
                self.check_if_comma_or_end(token.clone())?;
            }

            if token.kind == Token::Comma {
                self.pos += 1;
                expect_comma_or_end = false
            }

            if token.kind == Token::ParenClose {
                self.pos += 1;
                break;
            }

            if expect_comma_or_end {
                let (line, column) = self.error_handler.throw(token);

                return Err(
                    ParseError::UnexpectedToken(
                        String::from(""),
                        line,
                        column
                    )
                );
            }

            let expr = self.parse()?;
            args.push(expr);
            expect_comma_or_end = true
        }

        Ok(args)
    }

    fn check_if_comma_or_end(&mut self, token: TokenData) -> Result<(), ParseError> {
        if token.kind == Token::Comma {
            let (line, column) = self.error_handler.throw(token);

            return Err(
                ParseError::UnexpectedToken(
                    String::from(","),
                    line,
                    column
                )
            );
        }

        if token.kind == Token::ParenClose {
            let (line, column) = self.error_handler.throw(token);

            return Err(
                ParseError::UnexpectedToken(
                    String::from(")"),
                    line,
                    column
                )
            );
        }

        Ok(
            ()
        )
    }
}