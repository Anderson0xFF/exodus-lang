use crate::{
    scanner::{Keywords, Operator, Token},
    Lexer,
};

#[test]
fn tokenization_identifier() {
    let source = "x";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Identifier(String::from("x")));
    assert_eq!(lexer.next(), Token::EOF);
}

#[test]
fn tokenization_number_integer() {
    let source = "256";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Literal(crate::scanner::Literal::Integer(256)));
    assert_eq!(lexer.next(), Token::EOF);
}

#[test]
fn tokenization_number_floating() {
    let source = "73.81";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Literal(crate::scanner::Literal::Float(73.81)));
    assert_eq!(lexer.next(), Token::EOF);
}

#[test]
fn tokenization_type_i32() {
    let source = "i32";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Type(crate::types::Type::I32));
    assert_eq!(lexer.next(), Token::EOF);

}

#[test]
fn tokenization_type_f32() {
    let source = "f32";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Type(crate::types::Type::F32));
    assert_eq!(lexer.next(), Token::EOF);
}

#[test]
fn tokenization_let_keyword() {
    let source = "let x = 2;";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Keyword(Keywords::Let));
    assert_eq!(lexer.next(), Token::Identifier(String::from("x")));
    assert_eq!(lexer.next(), Token::Operator(Operator::ASSIGNMENT));
    assert_eq!(lexer.next(), Token::Literal(crate::scanner::Literal::Integer(2)));
    assert_eq!(lexer.next(), Token::Semicolon);
    assert_eq!(lexer.next(), Token::EOF);

}

#[test]
fn tokenization_if_keyword() {
    let source = "if (2 > 5) { }";

    let mut lexer = Lexer::tokenization("main", source);
    assert_eq!(lexer.next(), Token::Keyword(Keywords::If));
    assert_eq!(lexer.next(), Token::LParen);
    assert_eq!(lexer.next(), Token::Literal(crate::scanner::Literal::Integer(2)));
    assert_eq!(lexer.next(), Token::Operator(Operator::GT));
    assert_eq!(lexer.next(), Token::Literal(crate::scanner::Literal::Integer(5)));
    assert_eq!(lexer.next(), Token::RParen);
    assert_eq!(lexer.next(), Token::LBrace);
    assert_eq!(lexer.next(), Token::RBrace);
    assert_eq!(lexer.next(), Token::EOF);

}
