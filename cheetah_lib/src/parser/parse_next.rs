use super::parser::{ Parser, ParseResult };
use crate::types::expression::Expression;
use crate::types::tokens::Token;
use crate::types::expression::ExpressionDef;

impl Parser {
    pub fn parse_next(&mut self, expression: Expression) -> ParseResult {
        let next_token = self.get_token(self.pos)?;
        let mut continue_parse = true;
        let mut result = expression.clone();

        match next_token.kind {
            Token::Semicolon => {
                self.pos += 1;
            }
            Token::OperatorAddition => {
                result = self.binary_operations(
                    String::from("+"),
                    expression
                )?;
            }
            Token::ParenOpen => {
                match result.def {
                    ExpressionDef::Identifier(ref name, ref debug_info) => {
                        result = self.call_expression(name.clone(), debug_info.clone())?;
                    }
                    _ => {}
                }
            }
            Token::BracketOpen => {
                match result.def {
                    ExpressionDef::Identifier(ref _name, ref _debug_info) => {
                        result = self.member_expression(result)?;
                    }
                    _ => {}
                }
            }
            Token::Assign => {
                result = self.assign_expression(expression)?;
            }
            _ => continue_parse = false
        }

        let end_of_token = self.pos >= self.tokens.len();

        if continue_parse && !end_of_token {
            return self.parse_next(result);
        }

        Ok(result)
    }
}