// Contrato alineado con Lexer-main/src/token.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TipoToken {
    Keyword,
    Identifier,
    LiteralString,
    LiteralNumber,
    Operator,
    Punctuator,
    Error(String),
}

impl TipoToken {
    pub fn nombre(&self) -> &'static str {
        match self {
            TipoToken::Keyword => "KEYWORD",
            TipoToken::Identifier => "IDENTIFIER",
            TipoToken::LiteralString => "LITERAL_STRING",
            TipoToken::LiteralNumber => "LITERAL_NUMBER",
            TipoToken::Operator => "OPERATOR",
            TipoToken::Punctuator => "PUNCTUATOR",
            TipoToken::Error(_) => "ERROR",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ubicacion {
    pub linea: usize,
    pub columna: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub tipo: TipoToken,
    pub valor: String,
    pub ubicacion: Ubicacion,
}

impl Token {
    pub fn nuevo(tipo: TipoToken, valor: impl Into<String>, linea: usize, columna: usize) -> Self {
        Token {
            tipo,
            valor: valor.into(),
            ubicacion: Ubicacion { linea, columna },
        }
    }

    pub fn keyword(valor: impl Into<String>) -> Self {
        Self::nuevo(TipoToken::Keyword, valor, 0, 0)
    }

    pub fn identifier(valor: impl Into<String>) -> Self {
        Self::nuevo(TipoToken::Identifier, valor, 0, 0)
    }

    pub fn number(valor: impl Into<String>) -> Self {
        Self::nuevo(TipoToken::LiteralNumber, valor, 0, 0)
    }

    pub fn operator(valor: impl Into<String>) -> Self {
        Self::nuevo(TipoToken::Operator, valor, 0, 0)
    }

    pub fn punctuator(valor: impl Into<String>) -> Self {
        Self::nuevo(TipoToken::Punctuator, valor, 0, 0)
    }
}

pub const PALABRAS_RESERVADAS: &[&str] = &[
    "void", "float", "int", "return", "if", "else", "while", "break", "switch", "case", "default",
];
