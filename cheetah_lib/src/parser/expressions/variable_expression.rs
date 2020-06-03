use super::super::parser::{ Parser, ParseResult };
use crate::types::tokens::Token;
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::constant::Const;

impl Parser {
    pub fn variable_expression(&mut self) -> ParseResult {
        let name = self.get_token(self.pos)?;

        self.check_for_var_name();

        let tk = self.get_token(self.pos)?;

        let box_expr;

        match tk.kind {
            Token::Assign => {
                self.pos += 1;

                let expr = self.parse()?;

                box_expr = Box::new(expr);
            },
            Token::Semicolon | Token::NewLine => {
                box_expr = Box::new(
                    Expression::new(
                        ExpressionDef::Const(
                            Const::Undefined
                        )
                    )
                )
            }
            _ => {
                self.error_handler.throw(tk.clone());
                std::process::exit(1)
            }
        }

        Ok(
            Expression::new(
                ExpressionDef::VariableDecl(
                    name.value,
                    box_expr
                )
            )
        )
    }

    fn check_for_var_name(&mut self) {
        self.expect(self.pos, Token::Identifier);

        self.pos += 1;
    }
}