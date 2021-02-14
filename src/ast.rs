/// A primitive terminal value
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Integer(i32),
    Ident(String),
    Str(String),
    Bool(bool),
    None,
}

/// Unary and binary operators
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
    Exp,
    Not,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    Negation,
    MemberAccess,
}

/// A node of the AST, built by the parser, and evaluated by the runner
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
    MethodDef {
        name: String,
        for_type: String,
        args: Vec<String>,
        body: Vec<AstNode>,
    },
    Term(Term),
    Infix(Box<AstNode>, Op, Box<AstNode>),
    Unary(Op, Box<AstNode>),
    VarDeclaration(Term, Box<AstNode>),
    Assignment(Term, Box<AstNode>),
    WhileStmt(Box<AstNode>, Vec<AstNode>),
    ForStmt(String, Box<AstNode>, Vec<AstNode>),
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

pub fn infix(l: AstNode, op: Op, r: AstNode) -> AstNode {
    AstNode::Infix(Box::new(l), op, Box::new(r))
}

pub fn unary(op: Op, r: AstNode) -> AstNode {
    AstNode::Unary(op, Box::new(r))
}