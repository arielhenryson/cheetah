#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Unexpected Token
    UnexpectedToken(String, usize, usize),

    /// when no more token to handle
    EndOfTokens,
}