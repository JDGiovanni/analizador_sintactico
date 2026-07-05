#[derive(Debug, Clone)]
pub enum ASTNode {
    Identifier(String),
    Number(i32),
    Type(String),
    StringLiteral(String),

    Func {
        name: String,
        ret_type: Box<ASTNode>,
        args: Vec<ASTNode>,
        body: Box<ASTNode>,
    },

    Block {
        statements: Vec<ASTNode>,
    },

    Declaration {
        var_type: Box<ASTNode>,
        name: String,
        value: Box<ASTNode>,
    },

    IfStatement {
        condition: Box<ASTNode>,
        then_block: Box<ASTNode>,
        else_block: Option<Box<ASTNode>>,
    },

    FuncCall {
        name: String,
        args: Vec<ASTNode>,
    },

    Condition {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
}

impl ASTNode {
    pub fn mostrar(&self, nivel: usize) {
        let sangria = "   ".repeat(nivel); 

        match self {
            ASTNode::Identifier(txt) => println!("{}└── [Identifier] -> {}", sangria, txt),
            ASTNode::Number(num) => println!("{}└── [Number] -> {}", sangria, num),
            ASTNode::Type(t) => println!("{}└── [Type] -> {}", sangria, t),
            ASTNode::StringLiteral(txt) => println!("{}└── [StringLiteral] -> {}", sangria, txt),

            ASTNode::Func {name, ret_type, args: _, body} => {
                println!("{}├── [Func] -> name: '{}'", sangria, name);
                print!("{}│   ├── ret_type: ", sangria);
                ret_type.mostrar(0);
                println!("{}│   └── body:", sangria);
                body.mostrar(nivel + 2);
            }

            ASTNode::Block {statements} => {
                println!("{}├── [Block]", sangria);
                for stmt in statements {
                    stmt.mostrar(nivel + 2);
                }
            }

            ASTNode::Declaration { var_type, name, value } => {
                println!("{}├── [Declaration] -> name: '{}'", sangria, name);
                print!("{}│   ├── type: ", sangria);
                var_type.mostrar(0);
                print!("{}│   └── value: ", sangria);
                value.mostrar(0);
            }

            ASTNode::IfStatement { condition, then_block, else_block } => {
                println!("{}├── [If]", sangria);
                println!("{}│   ├── condition:", sangria);
                condition.mostrar(nivel + 2);
                println!("{}│   ├── then_block:", sangria);
                then_block.mostrar(nivel + 2);
                if let Some(eb) = else_block {
                    println!("{}│   └── else_block:", sangria);
                    eb.mostrar(nivel + 2);
                }
            }

            ASTNode::FuncCall { name, args } => {
                println!("{}└── [FuncCall] -> function: '{}'", sangria, name);
                for arg in args {
                    arg.mostrar(nivel + 2);
                }
            }

            ASTNode::Condition { left, operator, right } => {
                println!("{}├── [Condition] -> operator: '{}'", sangria, operator);
                print!("{}│   ├── left: ", sangria);
                left.mostrar(0);
                print!("{}│   └── right: ", sangria);
                right.mostrar(0);
            }
        }
    }
}