use super::tokens::Token;

#[derive(Debug,Clone, PartialEq)]
pub struct TokenData {
    pub kind: Token,
    pub value: String,
    pub position: TokenPos
}

#[derive(Debug,Clone, PartialEq)]
pub struct TokenPos {
    pub start: usize,
    pub end: usize
}