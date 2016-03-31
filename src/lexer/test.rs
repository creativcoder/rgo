use super::*;

#[test]
fn tokenize_delimiters() {
    assert_eq!(tokenize("("), vec![Token::OpenDelim(DelimToken::Paren)]);
    assert_eq!(tokenize(")"), vec![Token::CloseDelim(DelimToken::Paren)]);
    assert_eq!(tokenize("{"), vec![Token::OpenDelim(DelimToken::Brace)]);
    assert_eq!(tokenize("}"), vec![Token::CloseDelim(DelimToken::Brace)]);
    assert_eq!(tokenize("["), vec![Token::OpenDelim(DelimToken::Bracket)]);
    assert_eq!(tokenize("]"), vec![Token::CloseDelim(DelimToken::Bracket)]);
}

#[test]
fn tokenize_comma() {
    assert_eq!(tokenize(","), vec![Token::Comma]);
}

#[test]
fn tokenize_pipe_variants() {
    assert_eq!(tokenize("|"), vec![Token::Pipe]);
    assert_eq!(tokenize("||"), vec![Token::PipePipe]);
    assert_eq!(tokenize("|="), vec![Token::PipeEquals]);
}

#[test]
fn tokenize_plus_variants() {
    assert_eq!(tokenize("+"), vec![Token::Plus]);
    assert_eq!(tokenize("++"), vec![Token::Increment]);
    assert_eq!(tokenize("+="), vec![Token::PlusEquals]);
}

#[test]
fn tokenize_minus_variants() {
    assert_eq!(tokenize("-"), vec![Token::Minus]);
    assert_eq!(tokenize("--"), vec![Token::Decrement]);
    assert_eq!(tokenize("-="), vec![Token::MinusEquals]);
}

#[test]
fn tokenize_ident() {
    let test_ident = |s| {
        assert_eq!(tokenize(s), vec![Token::Ident(s.into())]);
    };

    test_ident("foo");
}

#[test]
fn tokenize_keywords() {
    let test_keyword = |s, k| {
        assert_eq!(tokenize(s), vec![Token::Keyword(k)]);
    };

    test_keyword("break", Keyword::Break);
    test_keyword("case", Keyword::Case);
    test_keyword("chan", Keyword::Chan);
    test_keyword("const", Keyword::Const);
    test_keyword("continue", Keyword::Continue);
    test_keyword("default", Keyword::Default);
    test_keyword("defer", Keyword::Defer);
    test_keyword("else", Keyword::Else);
    test_keyword("fallthrough", Keyword::Fallthrough);
    test_keyword("for", Keyword::For);
    test_keyword("func", Keyword::Func);
    test_keyword("go", Keyword::Go);
    test_keyword("goto", Keyword::Goto);
    test_keyword("if", Keyword::If);
    test_keyword("import", Keyword::Import);
    test_keyword("interface", Keyword::Interface);
    test_keyword("map", Keyword::Map);
    test_keyword("package", Keyword::Package);
    test_keyword("range", Keyword::Range);
    test_keyword("return", Keyword::Return);
    test_keyword("select", Keyword::Select);
    test_keyword("struct", Keyword::Struct);
    test_keyword("switch", Keyword::Switch);
    test_keyword("type", Keyword::Type);
    test_keyword("var", Keyword::Var);
}


#[test]
#[ignore] // This cannot work yet.
fn tokenize_hello() {
    let src = r#"
package main

import "fmt"

func main() {
	fmt.Println("Hello, rgo")
}
"#;

    let expected = [Token::Keyword(Keyword::Package),
                    Token::Whitespace,
                    Token::Ident("main".into()),
                    Token::Whitespace,
                    Token::Keyword(Keyword::Import),
                    Token::Whitespace,
                    Token::Literal(Literal::Str("fmt".into())),
                    Token::Whitespace,
                    Token::Keyword(Keyword::Func),
                    Token::Whitespace,
                    Token::Ident("main".into()),
                    Token::OpenDelim(DelimToken::Bracket),
                    Token::CloseDelim(DelimToken::Bracket),
                    Token::Whitespace,
                    Token::OpenDelim(DelimToken::Brace),
                    Token::Whitespace,
                    Token::Ident("fmt".into()),
                    Token::Dot,
                    Token::Ident("Println".into()),
                    Token::OpenDelim(DelimToken::Paren),
                    Token::Literal(Literal::Str("Hello, rgo".into())),
                    Token::CloseDelim(DelimToken::Paren),
                    Token::Whitespace,
                    Token::CloseDelim(DelimToken::Brace)];

    assert_eq!(tokenize(src), expected);
}
