use risl::parser::lexer::lex;

#[test]
fn lex_empty() {
    let source = "";
    let tokens = lex(source).collect::<Vec<_>>();
    assert!(tokens.is_empty());
}

#[test]
fn lex_simple_assignment() {
    let source = "let answer = 42;";
    let tokens = lex(source).collect::<Vec<_>>();
    assert!(!tokens.is_empty()); // TODO expected tokens
}
