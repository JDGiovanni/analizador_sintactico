// =========================================================================
// ESTRUCTURAS COMPARTIDAS (Esto debe adaptarlo el D1)
// =========================================================================
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeywordInt,         // "int"
    Identifier(String), // Nombres de variables (ej. "x")
    AssignOp,           // "="
    SemiColon,          // ";"
    // Añadan más tokens aquí (los suyos)
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    // ESTAS SON MIS ESTRUCTURAS (JHONMAR)
    Declaration {
        data_type: String,
        name: String,
        value: Option<Box<ASTNode>>, // Por si es "int x;" sin asignar
    },
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    // Nodo temporal para expresiones (EL D4 debe cambiarlo)
    Literal(i32),
    // El D1 e Interfaces de Control deben añadir más variantes aquí
}

// =========================================================================
// ESTRUCTURA PRINCIPAL DEL PARSER
// =========================================================================
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Crea una nueva instancia del Parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    // --- Métodos de navegación interna ---
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    // Valida que el token actual sea el esperado y avanza, si no, lanza un error sintáctico
    fn expect_token(&mut self, expected: Token, error_msg: &str) -> Result<(), String> {
        if self.current_token() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Error Sintáctico: {}. Encontrado: {:?}", error_msg, self.current_token()))
        }
    }

    // =========================================================================
    // HUECOS PARA USTEDES (Tareas de los D2 y 4)
    // =========================================================================
    
    // Lo llenará el D4 (Expresiones matemáticas/lógicas y llamadas)
    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        // Retorno provisional para que tu código compile y no dé error ahora
        Ok(ASTNode::Literal(0))
    }

    // Lo llenará el D2 (Estructuras de Control: If, Else, bucles)
    pub fn parse_control_structure(&mut self) -> Result<ASTNode, String> {
        Err("Estructuras de control no implementadas aún por Desarrollador 2".to_string())
    }

    // =========================================================================
    //  MI PARTE D3:jhonmar (Declaraciones y Asignaciones)
    // =========================================================================

    /// Analiza declaraciones de variables. Ej: "int x;" o "int x = 5;"
    pub fn parse_declaration(&mut self) -> Result<ASTNode, String> {
        let data_type = "int".to_string(); 
        self.advance(); // Consumir el token "int"

        // 1. Esperar el nombre de la variable (Identificador)
        let var_name = match self.current_token() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err("Se esperaba el nombre de la variable después del tipo de dato".to_string()),
        };
        self.advance(); // Consumir el identificador

        let mut initial_value = None;

        // 2. Verificar si viene una asignación opcional (ej: = 5)
        if self.current_token() == Some(&Token::AssignOp) {
            self.advance(); // Consumir el '='
            
            // Llama al D4 para resolver la expresión del valor
            let expr = self.parse_expression()?; 
            initial_value = Some(Box::new(expr));
        }

        // 3. Exigir obligatoriamente el ';' al final
        self.expect_token(Token::SemiColon, "Se esperaba ';' al final de la declaración")?;

        Ok(ASTNode::Declaration {
            data_type,
            name: var_name,
            value: initial_value,
        })
    }

    /// Analiza asignaciones a variables existentes. Ej: "x = 10;"
    pub fn parse_assignment(&mut self, var_name: String) -> Result<ASTNode, String> {
        // 1. Exigir y consumir el operador '='
        self.expect_token(Token::AssignOp, "Se esperaba '=' después del identificador")?;

        // 2. Aqui Llamo al D4 para resolver lo que está a la derecha del '='
        let expr = self.parse_expression()?;

        // 3. Exigir el ';' al final
        self.expect_token(Token::SemiColon, "Se esperaba ';' al final de la asignación")?;

        Ok(ASTNode::Assignment {
            name: var_name,
            value: Box::new(expr),
        })
    }
}
        })
    }
}
