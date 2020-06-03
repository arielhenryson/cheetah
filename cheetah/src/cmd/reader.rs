// load the source code from a file
pub fn read_source_code(path: &str) -> String {
    ::std::fs::read_to_string(path)
        .expect("Something went wrong reading the file")
}
