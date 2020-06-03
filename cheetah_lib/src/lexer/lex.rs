use logos::Logos;

use crate::types::tokens::Token;
use crate::types::token_data::{ TokenData, TokenPos };

type LexerResults = Vec<TokenData>;

pub fn lex_code(source_code: &String) -> Result<LexerResults, String> {
    let mut tokens = Token::lexer(
        source_code.as_str()
    );

    let mut tokens_vector = Vec::new();

    loop {
        if tokens.token == Token::End {
            break;
        }

        tokens_vector.push(TokenData {
            kind: tokens.token,
            value: tokens.slice().to_string(),
            position: TokenPos {
                start: tokens.range().start,
                end: tokens.range().end
            }
        });

        tokens.advance();
    }

    Ok(tokens_vector)
}
