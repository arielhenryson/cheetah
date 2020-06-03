use logos::{ Lexer, Source, Slice };

use crate::types::tokens::Token;

pub fn ignore_comments<'source, Src: Source<'source>>(lex: &mut Lexer<Token, Src>) {
    use logos::internal::LexerInternal;

    if lex.slice().as_bytes() == b"/*" {
        loop {
            match lex.read() {
                0    => return lex.token = Token::UnexpectedEndOfProgram,
                b'*' => {
                    if lex.next() == b'/' {
                        lex.bump(1);
                        break;
                    }
                },
                _ => lex.bump(1),
            }
        }
    }

    lex.advance();
}