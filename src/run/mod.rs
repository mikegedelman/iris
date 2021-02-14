mod builtins;
mod ops;
mod scope;
mod typing;

use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::{AstNode,Term};
use scope::Scope;
use typing::*;


#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    Str(String),
    Bool(bool),
    List(Vec<Value>),
    Function(Function),
    Some(Box<Value>),
    None,
    // Tuple(Vec<Value>, usize),
    // Dict(HashMap<Value, Value>),
    // Record
    // Enum{
    //     name: String,
    //     variant: String,
    //     data: Vec<Value>,
    // },
    // DoublePrecisionFloat(f64),
    // Undefined,
}

fn fn_call(name: &str, args: &Vec<AstNode>, scope: Rc<RefCell<Scope>>) -> Value {
    let evalled_args = args.iter().map(|arg| eval(arg, Rc::clone(&scope))).collect();

    match name {
        "print" => builtins::print(evalled_args),
        "list" => Value::List(evalled_args),
        "len" => builtins::len(evalled_args),
        "Some" => builtins::some(evalled_args),
        "unwrap" => builtins::unwrap(evalled_args),
        "is_some" => builtins::is_some(evalled_args),
        "is_none" => builtins::is_none(evalled_args),
        _ => {
            let s = scope.borrow();
            let mut func = if name.chars().next().unwrap().is_uppercase() {
                let first_arg = evalled_args.get(0)
                    .expect(&format!(
                        "Method {} has no args. Methods must have at least one arg.
                        NOTE: Capitalized function names are interpreted as methods by convention.",
                        name,
                    ));
                let typ = type_of(first_arg);
                s.get_method(name, typ.clone())
                    .expect(&format!(
                        "Unknown method {} for type {:?}
                        NOTE: Capitalized function names are interpreted as methods by convention.",
                        name,
                        typ,
                    ))
            } else {
                s.get_fn(name)
            };
 
            if func.args.len() != evalled_args.len() {
                panic!("Incorrect number of args for function \"{}\": got {}, expected {}", name,  evalled_args.len(), func.args.len());
            }
            for (idx, argname) in func.args.iter().enumerate() {
                if s.var_is_set(argname) {
                    panic!("function argument {} mirrors variable of the same name in outer scope", argname);
                }
                func.scope.declare_var(argname, evalled_args[idx].clone());
            }
            exec_fn(func)
        }
    }
}

fn exec_fn(func: Function) -> Value {
    let mut ret: Option<Value> = None;

    let body_len = func.body.len();
    let scope = Rc::new(RefCell::new(func.scope));
    for (idx, ast) in func.body.iter().enumerate() {
        match ast {
            _ => {
                // statement exec gets an Rc, which I think makes sense,
                // because statements may create a new function, which
                // would then need a new nested scope that refers to
                // this one
                let val = stmt(&ast, Rc::clone(&scope));
                if idx == (body_len - 1) {
                    ret = Some(val);
                }
            }
        };
    }

    match ret {
        Some(val) => val,
        None => Value::None,
    }
}


fn test_bool_val(v: Value) -> bool {
    match v {
        Value::Bool(true) => true,
        Value::Bool(false) => false,
        _ => panic!("Expected bool, got: {:?}", v),
    }
}

fn while_stmt(cond_expr: &AstNode, body: &Vec<AstNode>, scope: Rc<RefCell<Scope>>) {
    while test_bool_val(eval(cond_expr, Rc::clone(&scope))) {
        stmt_body(body, Rc::clone(&scope));
    }
}

fn exec_if(
    cond_expr: &AstNode,
    body: &Vec<AstNode>,
    else_if: &Vec<AstNode>,
    else_body: &Vec<AstNode>,
    scope: Rc<RefCell<Scope>>
) -> Value {
    if test_bool_val(eval(cond_expr, Rc::clone(&scope))) {
        return stmt_body(body, Rc::clone(&scope));
    }
    for try_else_if in else_if {
        let (cond_expr, body) = match try_else_if {
            AstNode::ElseIf{ cond_expr, body } => (cond_expr, body),
            _ => panic!("expected ElseIf, got {:?}", try_else_if),
        };
        if test_bool_val(eval(cond_expr, Rc::clone(&scope))) {
            return stmt_body(body, Rc::clone(&scope));
        }
    }
    stmt_body(else_body, Rc::clone(&scope))

}

fn stmt_body(body: &Vec<AstNode>, scope: Rc<RefCell<Scope>>) -> Value {
    let mut ret: Option<Value> = None;
    let body_len = body.len();
    for (idx, ast) in body.iter().enumerate() {
        let val = stmt(&ast, Rc::clone(&scope));
        if idx == (body_len - 1) {
            ret = Some(val);
        }
    }
    match ret {
        Some(val) => val,
        None => Value::None,
    }
}


fn eval(ast: &AstNode, scope: Rc<RefCell<Scope>>) -> Value { // : &mut Scope) -> Value {
    match ast {
        AstNode::FnCall{ name, args } => fn_call(name, args, scope),
        AstNode::FnDef{ name, args, body } => {
            Value::Function(
                Function {
                    name: name.to_string(),
                    args: args.to_vec(),
                    body: body.to_vec(),
                    scope: Scope::nest(scope, name),
                }
            )
        },
        AstNode::If{ cond_expr, body, else_if, else_body } => {
            exec_if(cond_expr, body, else_if, else_body, scope)
        },
        AstNode::Infix(lhs, op, rhs) => ops::infix(
            eval(lhs, Rc::clone(&scope)), op.clone(), eval(rhs, Rc::clone(&scope))
        ),
        AstNode::Unary(op, rhs) => ops::unary(op.clone(), eval(rhs, Rc::clone(&scope))),
        AstNode::Term(Term::Str(x)) => Value::Str(x.to_string()),
        AstNode::Term(Term::Integer(x)) => Value::Integer(*x),
        AstNode::Term(Term::Bool(x)) => Value::Bool(*x),
        AstNode::Term(Term::None) => Value::None,
        // AstNode::Term(Term::DoublePrecisionFloat(x)) => Value::DoublePrecisionFloat(*x),
        AstNode::Term(Term::Ident(var)) => {
            let s = scope.borrow();
            s.get_var(var).expect(&format!("Unable to resolve var {:?}", var))
        },
        _ => panic!("Unexpected ast {:?}", ast),
    }
}

fn stmt(ast: &AstNode, scope: Rc<RefCell<Scope>>) -> Value {
    match ast {
        AstNode::VarDeclaration(Term::Ident(var), astbox) => {
            let val = eval(astbox, Rc::clone(&scope));
            let mut s = scope.borrow_mut();
            s.declare_var(var, val);
            Value::None
        },
        AstNode::Assignment(Term::Ident(var), astbox) => {
            let val = eval(astbox, Rc::clone(&scope));
            let mut s = scope.borrow_mut();
            s.set_var(var, val);
            Value::None
        },
        AstNode::MethodDef{ name, for_type, args, body } => {
            if !name.chars().next().unwrap().is_uppercase() {
                panic!("Method names must be capitalized by convention. Got: {}", name);
            }
            let method = Function {
                name: name.to_string(),
                args: args.to_vec(),
                body: body.to_vec(),
                scope: Scope::nest(Rc::clone(&scope), name),
            };
            let mut s = scope.borrow_mut();
            s.declare_method(name, read_type_definition(for_type), method);
            Value::None
        },
        AstNode::WhileStmt(cond, body) => {
            while_stmt(cond, body, Rc::clone(&scope));
            Value::None
        },
        _ => eval(ast, scope),
    }
}

pub fn run(ast_list: Vec<AstNode>) {
    let global_scope = Rc::new(RefCell::new(Scope::new(String::from("<top level>"))));
    for ast_node in ast_list {
        stmt(&ast_node, Rc::clone(&global_scope));
    }
}