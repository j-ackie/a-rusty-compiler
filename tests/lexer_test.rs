use compiler::lexer;

#[test]
fn test_tokenize_empty_source() {
    let source = "".to_string();
    let tokens = lexer::tokenize(source, true).unwrap();
    assert_eq!(tokens.len(), 0);
}
