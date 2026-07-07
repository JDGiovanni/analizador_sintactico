mod parser;
mod token;

use parser::{Parser, ASTNode};
use token::Token;

fn main() {
    // Ejemplo: if (b != 0) { return a; } else { return 0; }
    let tokens = vec![
        Token::keyword("if"),
        Token::punctuator("("),
        Token::identifier("b"),
        Token::operator("!="),
        Token::number("0"),
        Token::punctuator(")"),
        Token::punctuator("{"),
        Token::keyword("return"),
        Token::identifier("a"),
        Token::punctuator(";"),
        Token::punctuator("}"),
        Token::keyword("else"),
        Token::punctuator("{"),
        Token::keyword("return"),
        Token::number("0"),
        Token::punctuator(";"),
        Token::punctuator("}"),
    ];

    let mut parser = Parser::new(tokens);

    match parser.parse_if() {
        Ok(ast) => {
            println!("AST generado correctamente:\n");
            ast.mostrar(0);
        }
        Err(e) => eprintln!("Error sintáctico: {}", e),
    }

    println!("\n--- Ejemplo while ---\n");

    let tokens_while = vec![
        Token::keyword("while"),
        Token::punctuator("("),
        Token::number("1"),
        Token::operator("=="),
        Token::number("1"),
        Token::punctuator(")"),
        Token::punctuator("{"),
        Token::keyword("break"),
        Token::punctuator(";"),
        Token::punctuator("}"),
    ];

    let mut parser = Parser::new(tokens_while);
    match parser.parse_while() {
        Ok(ast) => ast.mostrar(0),
        Err(e) => eprintln!("Error sintáctico: {}", e),
    }
}
