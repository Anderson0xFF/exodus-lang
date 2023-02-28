use crate::{
    scanner::{Keywords, Operator, Token},
    Lexer,
};

#[test]
fn tokenization_identifier() {
    let source = "x";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Identifier(String::from("x")));
}

#[test]
fn tokenization_number_integer() {
    let source = "256";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Integer(256));
}

#[test]
fn tokenization_number_floating() {
    let source = "73.81";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Floating(73.81));
}

#[test]
fn tokenization_type_i32() {
    let source = "i32";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Type(crate::types::Type::I32));
}

#[test]
fn tokenization_let_keyword() {
    let source = "let x = 2;";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Keyword(Keywords::Let));
    assert_eq!(lexer.next(), Token::Identifier(String::from("x")));
    assert_eq!(lexer.next(), Token::Operator(Operator::ASSIGNMENT));
    assert_eq!(lexer.next(), Token::Integer(2));
    assert_eq!(lexer.next(), Token::Semicolon);
}

#[test]
fn tokenization_if_keyword() {
    let source = "if (2 > 5) { }";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Keyword(Keywords::If));
    assert_eq!(lexer.next(), Token::LP);
    assert_eq!(lexer.next(), Token::Integer(2));
    assert_eq!(lexer.next(), Token::Operator(Operator::GT));
    assert_eq!(lexer.next(), Token::Integer(5));
    assert_eq!(lexer.next(), Token::RP);
    assert_eq!(lexer.next(), Token::LB);
    assert_eq!(lexer.next(), Token::RB);
}
