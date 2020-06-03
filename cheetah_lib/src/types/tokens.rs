use logos::Logos;

use crate::lexer::ignore_comments::ignore_comments;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Token {
    // Logos requires that we define two default variants,
    // one for end of input source,
    #[end]
    End,

    // ...and one for errors. Those can be named anything
    // you wish as long as the attributes are there.
    #[error]
    Error,

    #[token = ";"]
    Semicolon,

    #[token = "+"]
    OperatorAddition,

    #[token = "return"]
    KeywordReturn,

    #[token = "if"]
    KeywordIf,

    #[token = "else"]
    KeywordElse,

    // Or regular code_gen.
    #[regex = "[0-9]+"]
    Number,

    #[token = "function"]
    DeclarationFunction,

    #[token = "("]
    ParenOpen,

    #[token = ")"]
    ParenClose,

    #[token = "{"]
    BraceOpen,

    #[token = "}"]
    BraceClose,

    #[token = "["]
    BracketOpen,

    #[token = "]"]
    BracketClose,

    #[token = "="]
    Assign,

    #[token = ","]
    Comma,

    #[regex = "[a-zA-Z_$][a-zA-Z0-9_$]*"]
    Identifier,

    #[token = "let"]
    DeclarationVariable,

    #[regex = "[\n]"]
    NewLine,

    #[regex = "//[^\n]*"]
    #[token = "/*"]
    #[callback = "ignore_comments"]
    UnexpectedToken,
    UnexpectedEndOfProgram,
}


