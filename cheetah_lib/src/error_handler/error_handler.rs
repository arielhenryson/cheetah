use crate::error_handler::source_code::SourceCode;
use crate::types::token_data::TokenData;

#[derive(Debug, Clone)]
pub struct ErrorHandler {
    // The current source code path
    pub source_code: SourceCode
}

impl ErrorHandler {
    pub fn new(source_code: SourceCode) -> Self {
        Self {
            source_code
        }
    }

    fn get_line_and_column_number(&mut self, error_pos: usize) -> (usize, usize) {
        let mut pos = 0;
        let mut line_number = 1;
        let mut column_number = 1;

        for char in self.source_code.content.chars() {
            if pos == error_pos {
                break;
            }

            if char == "\n".parse().unwrap() {
                line_number += 1;
                column_number = 0;
            }

            column_number += 1;
            pos += 1;
        }

        (line_number, column_number)
    }

    pub fn error_loc(&mut self, error_pos: usize) -> (String, usize, usize) {
        let (line, column) = self.get_line_and_column_number(error_pos);

        (String::from(
            format!("at line {} column {} at {}", line, column, self.source_code.file_name.clone())
        ),
         line,
         column)
    }

    pub fn throw(&mut self, token: TokenData) -> (usize, usize) {
        let (loc, line, column) = self.error_loc(token.position.start);

        eprintln!("Unexpected token {:?} -> `{}` {}",
                  token.kind,
                  token.value,
                  loc
        );

        (line, column)
    }
}