use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::HashMap;

use crate::ast::{AstNode,Term,Op};
use crate::builtins;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    Str(String),
    Bool(bool),
    List(Vec<Value>),
    Function(Function),
    // Option(Box<OptionValue>),
    // Option(
    None,
    // DoublePrecisionFloat(f64),
    // Undefined,
}

#[derive(Clone, Debug)]
enum OptionValue {
    None,
    Some(Value),
}


#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<AstNode>,
    pub scope: Scope,
}

#[derive(Clone, Debug)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    context: String,
    vars: HashMap<String, Value>,
}

impl Scope {
    pub fn new(context: String) -> Scope {
        Scope{ context, vars: HashMap::new(), parent: None, }
    }

    /// Associated fn instead of a method because we want to clone an Rc
    /// to an existing RefCell, so we accept that instead
    pub fn nest(parent: Rc<RefCell<Scope>>, context: &str) -> Scope  {
        Scope {
            parent: Some(Rc::clone(&parent)),
            context: context.to_string(),
            vars: HashMap::new(),
        }
    }

    pub fn declare_var(&mut self, name: &str, val: Value) {
        assert!(!self.vars.contains_key(name), format!("can't redeclare var {}", name));
        self.vars.insert(name.to_string(), val);
    }

    pub fn set_var(&mut self, name: &str, val: Value) {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), val);
        } else {
            match &self.parent {
                Some(p) => {
                    let mut parent = p.borrow_mut();
                    parent.set_var(name, val);
                },
                None => panic!("can't assign to undeclared var \"{}\" | context: {}", name, self.context),
            }
        }
    }

    pub fn get_var(&self, name: &str) -> Value {
        match self.vars.get(name) {
            Some(x) => x.clone(),
            None => match &self.parent {
                        Some(p) => {
                            let parent = p.borrow();
                            parent.get_var(name)
                        },
                        None => panic!("unknown var: \"{}\" | context: {}", name, self.context),
                    }
        }
    }

    // Not sure why this is returning Vec<&String> but .collect() was angry with anything else
    pub fn all_var_names(&self) -> Vec<&String> {
        self.vars.keys().collect()
    }

    pub fn var_is_set(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }
}

fn fn_call(name: &str, args: &Vec<AstNode>, scope: Rc<RefCell<Scope>>) -> Value {
    let evalled_args = args.iter().map(|arg| eval(arg, Rc::clone(&scope))).collect();

    match name {
        "print" => builtins::print(evalled_args),
        "list" => Value::List(evalled_args),
        "len" => builtins::len(evalled_args),
        _ => {
            let s = scope.borrow();
            let maybe_func = s.get_var(name);
            let mut func = match maybe_func {
                Value::Function(f) => f,
                _ => panic!("{:?} is not a function", maybe_func),
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


fn exec_if(cond_expr: &AstNode, body: &Vec<AstNode>, else_if: &Vec<AstNode>, else_body: &Vec<AstNode>, scope: Rc<RefCell<Scope>>) {
    if test_bool_val(eval(cond_expr, Rc::clone(&scope))) {
        stmt_body(body, Rc::clone(&scope));
        return;
    }
    for try_else_if in else_if {
        let (cond_expr, body) = match try_else_if {
            AstNode::ElseIf{ cond_expr, body } => (cond_expr, body),
            _ => panic!("expected ElseIf, got {:?}", try_else_if),
        };
        if test_bool_val(eval(cond_expr, Rc::clone(&scope))) {
            stmt_body(body, Rc::clone(&scope));
            return;
        }
    }
    stmt_body(else_body, Rc::clone(&scope));

}

fn stmt_body(body: &Vec<AstNode>, scope: Rc<RefCell<Scope>>) {
    for ast in body {
        stmt(ast, Rc::clone(&scope));
    }
}


// fn arithmetic(lhs: Value, op: Op, rhs: Value) -> Value {
//     match lhs {
//         Value::Integer(i) => arith_int(i, op, rhs),
//         // Value::DoublePrecisionFloat(f) => arith_float(f, op, rhs),
//         Value::Str(s) => arith_str(s, op, rhs),
//         Value::Bool(_) => panic!("todo implement bool arith"), // arith_bool(s, op, rhs),
//         Value::Function(_) => panic!("can't {:?} on function", op), // arith_bool(s, op, rhs),
//         Value::List(_) => panic!("todo implement list arith"),
//         // Value::Undefined => panic!("Can't {:?} undefined and {:?}", op, rhs),
//         Value::None => panic!("Can't {:?} None and {:?}", op, rhs),
//     }
// }

// fn arith_int(a: i32, op: Op, rhs: Value) -> Value {
//     let res = match rhs {
//         Value::Integer(b) => match op {
//             Op::Add => a + b,
//             Op::Sub => a - b,
//             Op::Mul => a * b,
//             Op::Div => a / b,
//             Op::Shr => a >> b,
//             Op::Shl => a << b,
//             Op::And => a & b,
//             Op::Or => a | b,
//             Op::Xor => a ^ b,
//             Op::Mod => a % b,
//             Op::Exp => a.pow(b as u32),
//         },
//         _ => panic!("Can't {:?} int {} with {:?}", op, a, rhs),
//     };
//     Value::Integer(res)
// }

// fn arith_float(a: f64, op: Op, rhs: Value) -> Value {
//     let res = match rhs {
//         Value::DoublePrecisionFloat(b) => match op {
//             Op::Add => a + b,
//             Op::Sub => a - b,
//             Op::Mul => a * b,
//             Op::Div => a / b,
//             _ => panic!("Operation {:?} not defined for float", op),
//         },
//         _ => panic!("Can't {:?} float {} with {:?}", op, a, rhs),
//     };
//     Value::DoublePrecisionFloat(res)
// }

// fn arith_str(a: String, op: Op, rhs: Value) -> Value {
//     let res = match rhs {
//         Value::Str(b) => match op {
//             Op::Add => a + &b,
//             _ => panic!("operator {:?} not defined for string", op),
//         },
//         _ => panic!("Can't {:?} string {} with {:?}", op, a, rhs),
//     };
//     Value::Str(res)
// }

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
        // AstNode::Arithmetic(lhs, op, rhs) => arithmetic(eval(lhs, scope), op.clone(), eval(rhs, scope)),
        AstNode::Term(Term::Str(x)) => Value::Str(x.to_string()),
        AstNode::Term(Term::Integer(x)) => Value::Integer(*x),
        AstNode::Term(Term::Bool(x)) => Value::Bool(*x),
        // AstNode::Term(Term::DoublePrecisionFloat(x)) => Value::DoublePrecisionFloat(*x),
        AstNode::Term(Term::Ident(var)) => {
            let s = scope.borrow();
            s.get_var(var)
        }
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
        AstNode::If{ cond_expr, body, else_if, else_body } => {
            exec_if(cond_expr, body, else_if, else_body, scope);
            Value::None // todo
        }
        _ => eval(ast, scope),
    }
}

pub fn run(ast_list: Vec<AstNode>) {
    let global_scope = Rc::new(RefCell::new(Scope::new(String::from("<top level>"))));
    for ast_node in ast_list {
        stmt(&ast_node, Rc::clone(&global_scope));
    }
}