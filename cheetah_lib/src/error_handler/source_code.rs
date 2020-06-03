#[derive(Debug, Clone)]
pub struct SourceCode {
    pub file_name: String,
    pub content: String
}

impl SourceCode {
    pub fn new(file_name: String, content: String) -> Self {
        Self {
            file_name,
            content
        }
    }
}