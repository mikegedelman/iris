#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Integer(i32),
    // DoublePrecisionFloat(f64),
    Ident(String),
    Str(String),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Exp,
}

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Var {
//     type_: String,
//     name: String,
// }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AstNode {
    FnCall {
        name: String,
        args: Vec<AstNode>,
    },
    FnDef {
        name: String,
        args: Vec<String>,
        body: Vec<AstNode>,
    },
    Term(Term),
    Arithmetic(Box<AstNode>, Op, Box<AstNode>),
    VarDeclaration(Term, Box<AstNode>),
    Assignment(Term, Box<AstNode>),
    Return(Box<AstNode>),
    If {
        cond_expr: Box<AstNode>,
        body: Vec<AstNode>,
        else_if: Vec<AstNode>, // Should contain ElseIf nodes
        else_body: Vec<AstNode>,
    },
    ElseIf {
        cond_expr: Box<AstNode>,
        body: Vec<AstNode>,
    },
}
