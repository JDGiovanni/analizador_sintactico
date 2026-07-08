use crate::token::{Token, TipoToken};
use super::ast::ASTNode;
use super::parser::Parser;

impl Parser {
    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_condition()
    }

    pub fn parse_condition(&mut self) -> Result<ASTNode, String> {
        let left = self.parse_term()?;

        if let Some(token) = self.current_token() {
            if token.tipo == TipoToken::Operator && (token.valor == ">" || token.valor == "<" || token.valor == "==" || token.valor == "!=") {
                let operator = token.valor.clone();
                self.advance();

                let right = self.parse_term()?;
                return Ok(ASTNode::Condition {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                });
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_factor()?;

        while self.match_operator("+") || self.match_operator("-") {
            let operator = self.current_token().unwrap().valor.clone();
            self.advance();

            let right = self.parse_factor()?;
            
            expr = ASTNode::Condition {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_call()?;

        while self.match_operator("*") || self.match_operator("/") {
            let operator = self.current_token().unwrap().valor.clone();
            self.advance();

            let right = self.parse_call()?;
            
            expr = ASTNode::Condition {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_call(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_primary()?;

        if let ASTNode::Identifier(ref name) = expr {
            if self.match_punctuator("(") {
                self.advance();
                
                let mut args = Vec::new();
                if !self.match_punctuator(")") {
                    loop {
                        args.push(self.parse_expression()?);
                        
                        if self.match_punctuator(",") {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                
                self.expect_punctuator(")", "Se esperaba ')' tras los argumentos de la función")?;
                return Ok(ASTNode::FuncCall { name: name.clone(), args });
            }
        }

        Ok(expr)
    }

    pub fn parse_primary(&mut self) -> Result<ASTNode, String> {
        let token = match self.current_token() {
            Some(t) => t.clone(),
            None => return Err("Se esperaba una expresión, pero se llegó al final del archivo".to_string()),
        };

        match token.tipo {
            TipoToken::Identifier => {
                self.advance();
                Ok(ASTNode::Identifier(token.valor))
            }
            TipoToken::LiteralNumber => {
                self.advance();
                let num: i32 = token.valor.parse().map_err(|_| {
                    format!("Número inválido en expresión: {}", token.valor)
                })?;
                Ok(ASTNode::Number(num))
            }
            TipoToken::LiteralString => { 
                self.advance();
                Ok(ASTNode::StringLiteral(token.valor))
            }
            TipoToken::Punctuator if token.valor == "(" => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect_punctuator(")", "Se esperaba ')' tras la expresión agrupada")?;
                Ok(expr)
            }
            other => Err(format!(
                "Se esperaba identificador, número, cadena o '(' en expresión. Encontrado: {:?}",
                other
            )),
        }
    }
}