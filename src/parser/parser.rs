use crate::token::{Token, TipoToken};
use super::ast::ASTNode;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn match_keyword(&self, kw: &str) -> bool {
        matches!(
            self.current_token(),
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) if valor == kw
        )
    }

    fn match_punctuator(&self, p: &str) -> bool {
        matches!(
            self.current_token(),
            Some(Token {
                tipo: TipoToken::Punctuator,
                valor,
                ..
            }) if valor == p
        )
    }

    fn match_operator(&self, op: &str) -> bool {
        matches!(
            self.current_token(),
            Some(Token {
                tipo: TipoToken::Operator,
                valor,
                ..
            }) if valor == op
        )
    }

    fn expect_keyword(&mut self, kw: &str, error_msg: &str) -> Result<(), String> {
        if self.match_keyword(kw) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Error Sintáctico: {}. Encontrado: {:?}",
                error_msg,
                self.current_token()
            ))
        }
    }

    fn expect_punctuator(&mut self, p: &str, error_msg: &str) -> Result<(), String> {
        if self.match_punctuator(p) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Error Sintáctico: {}. Encontrado: {:?}",
                error_msg,
                self.current_token()
            ))
        }
    }

    // =========================================================================
    // D4 — Expresiones (stub provisional)
    // =========================================================================

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        match self.current_token() {
            Some(Token {
                tipo: TipoToken::Identifier,
                valor,
                ..
            }) => {
                self.advance();
                Ok(ASTNode::Identifier(valor.clone()))
            }
            Some(Token {
                tipo: TipoToken::LiteralNumber,
                valor,
                ..
            }) => {
                self.advance();
                let num: i32 = valor.parse().map_err(|_| {
                    format!("Número inválido en expresión: {}", valor)
                })?;
                Ok(ASTNode::Number(num))
            }
            other => Err(format!(
                "Se esperaba identificador o número. Encontrado: {:?}",
                other
            )),
        }
    }

    fn parse_condition(&mut self) -> Result<ASTNode, String> {
        let left = self.parse_primary()?;

        let operator = match self.current_token() {
            Some(Token {
                tipo: TipoToken::Operator,
                valor,
                ..
            }) => valor.clone(),
            _ => {
                return Err(format!(
                    "Se esperaba operador en la condición. Encontrado: {:?}",
                    self.current_token()
                ))
            }
        };
        self.advance();

        let right = self.parse_primary()?;

        Ok(ASTNode::Condition {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    // =========================================================================
    // D2 — Estructuras de control: If, Block, While
    // =========================================================================

    pub fn parse_control_structure(&mut self) -> Result<ASTNode, String> {
        match self.current_token() {
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) if valor == "if" => self.parse_if(),
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) if valor == "while" => self.parse_while(),
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) if valor == "break" => self.parse_break(),
            other => Err(format!(
                "Estructura de control no reconocida. Encontrado: {:?}",
                other
            )),
        }
    }

    pub fn parse_if(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("if", "Se esperaba 'if'")?;

        let has_parens = self.match_punctuator("(");
        if has_parens {
            self.advance();
        }

        let condition = self.parse_condition()?;

        if has_parens {
            self.expect_punctuator(")", "Se esperaba ')' después de la condición")?;
        }

        let then_block = self.parse_block()?;

        let else_block = if self.match_keyword("else") {
            self.advance();
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        Ok(ASTNode::IfStatement {
            condition: Box::new(condition),
            then_block: Box::new(then_block),
            else_block,
        })
    }

    pub fn parse_while(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("while", "Se esperaba 'while'")?;
        self.expect_punctuator("(", "Se esperaba '(' después de 'while'")?;

        let condition = self.parse_condition()?;
        self.expect_punctuator(")", "Se esperaba ')' después de la condición del while")?;

        let body = self.parse_block()?;

        Ok(ASTNode::WhileStatement {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    pub fn parse_break(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("break", "Se esperaba 'break'")?;
        self.expect_punctuator(";", "Se esperaba ';' después de 'break'")?;
        Ok(ASTNode::Break)
    }

    pub fn parse_block(&mut self) -> Result<ASTNode, String> {
        self.expect_punctuator("{", "Se esperaba '{' al inicio del bloque")?;

        let mut statements = Vec::new();
        while !self.match_punctuator("}") && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.expect_punctuator("}", "Se esperaba '}' al final del bloque")?;

        Ok(ASTNode::Block { statements })
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        match self.current_token() {
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) if valor == "if" => self.parse_if(),
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) if valor == "while" => self.parse_while(),
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) if valor == "break" => self.parse_break(),
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) if matches!(valor.as_str(), "int" | "float" | "void") => self.parse_declaration(),
            Some(Token {
                tipo: TipoToken::Identifier,
                ..
            }) => {
                let name = self.current_token().unwrap().valor.clone();
                let is_assignment = self
                    .tokens
                    .get(self.position + 1)
                    .map(|t| t.tipo == TipoToken::Operator && t.valor == "=")
                    .unwrap_or(false);

                if is_assignment {
                    self.advance();
                    self.parse_assignment(name)
                } else {
                    Err(format!(
                        "Sentencia no reconocida después de identificador '{}'",
                        name
                    ))
                }
            }
            other => Err(format!("Sentencia no reconocida. Encontrado: {:?}", other)),
        }
    }

    // =========================================================================
    // D3 — Declaraciones y asignaciones (adaptadas al token del lexer)
    // =========================================================================

    pub fn parse_declaration(&mut self) -> Result<ASTNode, String> {
        let type_name = match self.current_token() {
            Some(Token {
                tipo: TipoToken::Keyword,
                valor,
                ..
            }) => valor.clone(),
            _ => {
                return Err(
                    "Se esperaba un tipo de dato (int, float, void)".to_string(),
                )
            }
        };
        self.advance();

        let var_name = match self.current_token() {
            Some(Token {
                tipo: TipoToken::Identifier,
                valor,
                ..
            }) => valor.clone(),
            _ => {
                return Err(
                    "Se esperaba el nombre de la variable después del tipo de dato".to_string(),
                )
            }
        };
        self.advance();

        let value = if self.match_operator("=") {
            self.advance();
            self.parse_expression()?
        } else {
            ASTNode::Number(0)
        };

        self.expect_punctuator(";", "Se esperaba ';' al final de la declaración")?;

        Ok(ASTNode::Declaration {
            var_type: Box::new(ASTNode::Type(type_name)),
            name: var_name,
            value: Box::new(value),
        })
    }

    pub fn parse_assignment(&mut self, var_name: String) -> Result<ASTNode, String> {
        if !self.match_operator("=") {
            return Err(format!(
                "Se esperaba '=' después de '{}'. Encontrado: {:?}",
                var_name,
                self.current_token()
            ));
        }
        self.advance();

        let expr = self.parse_expression()?;
        self.expect_punctuator(";", "Se esperaba ';' al final de la asignación")?;

        Ok(ASTNode::Assignment {
            name: var_name,
            value: Box::new(expr),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    fn tokens_if_simple() -> Vec<Token> {
        vec![
            Token::keyword("if"),
            Token::identifier("suma"),
            Token::operator(">"),
            Token::number("0"),
            Token::punctuator("{"),
            Token::keyword("return"),
            Token::identifier("suma"),
            Token::punctuator(";"),
            Token::punctuator("}"),
        ]
    }

    fn tokens_if_else() -> Vec<Token> {
        vec![
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
        ]
    }

    fn tokens_while() -> Vec<Token> {
        vec![
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
        ]
    }

    #[test]
    fn parse_if_sin_parentesis() {
        let mut parser = Parser::new(tokens_if_simple());
        let ast = parser.parse_if().expect("debe parsear if");
        assert!(matches!(ast, ASTNode::IfStatement { .. }));
    }

    #[test]
    fn parse_if_con_else() {
        let mut parser = Parser::new(tokens_if_else());
        let ast = parser.parse_if().expect("debe parsear if/else");
        if let ASTNode::IfStatement { else_block, .. } = ast {
            assert!(else_block.is_some());
        } else {
            panic!("se esperaba IfStatement");
        }
    }

    #[test]
    fn parse_while_con_break() {
        let mut parser = Parser::new(tokens_while());
        let ast = parser.parse_while().expect("debe parsear while");
        if let ASTNode::WhileStatement { body, .. } = ast {
            if let ASTNode::Block { statements } = *body {
                assert_eq!(statements.len(), 1);
                assert!(matches!(statements[0], ASTNode::Break));
            } else {
                panic!("cuerpo del while debe ser Block");
            }
        } else {
            panic!("se esperaba WhileStatement");
        }
    }
}
