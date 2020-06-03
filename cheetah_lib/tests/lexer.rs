use cheetah_lib::lexer::lex::lex_code;

#[test]
fn lex_function() {
    let input = String::from("function() {}");

    let res = lex_code(&input).unwrap();

    let tokens_length = res.len();

    assert_eq!(tokens_length, 5);
}

#[test]
fn lex_return() {
    let input = String::from("return 10 + 10");

    let res = lex_code(&input).unwrap();

    let tokens_length = res.len();

    assert_eq!(tokens_length, 4);
}

#[test]
fn lex_return2() {
    let input = String::from("return 10 + 10;");

    let res = lex_code(&input).unwrap();

    let tokens_length = res.len();

    assert_eq!(tokens_length, 5);
}