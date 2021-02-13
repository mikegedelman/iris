use crate::ast::{AstNode,Term,Op};
use std::collections::HashMap;

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
}

#[derive(Clone, Debug)]
pub struct Scope {
    context: String,
    vars: HashMap<String, Value>,
}

impl Scope {
    pub fn new(context: String) -> Scope {
        Scope{ context, vars: HashMap::new() }
    }

    pub fn nest(&self, context: String) -> Scope  {
        let mut ret = self.clone();
        ret.context = context;
        ret
    }

    pub fn declare_var(&mut self, name: &str, val: Value) {
        assert!(!self.vars.contains_key(name), format!("can't redeclare var {}", name));
        self.vars.insert(name.to_string(), val);
    }

    pub fn set_var(&mut self, name: &str, val: Value) {
        assert!(self.vars.contains_key(name), format!("can't assign to var {}: not declared", name));
        self.vars.insert(name.to_string(), val);
    }

    pub fn get_var(&mut self, name: &str) -> Value {
        let val = match self.vars.get(name) {
            Some(x) => x,
            None => panic!("unknown var: \"{}\" | context: {}", name, self.context),
        };
        val.clone()
    }

    pub fn var_is_set(&mut self, name: &str) -> bool {
        self.vars.contains_key(name)
    }
}

pub fn fn_call(name: &str, args: &Vec<AstNode>, scope: &mut Scope) -> Value {
    let evalled_args = args.iter().map(|arg| eval(arg, scope)).collect();

    match name {
        "print" => builtins::print(evalled_args),
        // "list" => builtins::list(evalled_args, scope),
        "list" => Value::List(evalled_args),
        "len" => builtins::len(evalled_args),
        _ => {
            let maybe_func = scope.get_var(name);
            let func = match maybe_func {
                Value::Function(f) => f,
                _ => panic!("{:?} is not a function", maybe_func),
            };
            let mut inner_scope = scope.nest(format!("function \"{}\"", name));

            if func.args.len() != evalled_args.len() {
                panic!("Incorrect number of args for function \"{}\": got {}, expected {}", name,  evalled_args.len(), func.args.len());
            }
            for (idx, argname) in func.args.iter().enumerate() {
                if inner_scope.var_is_set(argname) {
                    panic!("function argument {} mirrors variable of the same name in outer scope", argname);
                }
                inner_scope.declare_var(argname, evalled_args[idx].clone());
            }
            exec_fn(func, &mut inner_scope)
            // For closures: check for variables in inner scope to be lifted back out?
        }
    }
}

pub fn exec_fn(func: Function, scope: &mut Scope) -> Value {
    let mut ret: Option<Value> = None;

    let body_len = func.body.len();
    for (idx, ast) in func.body.iter().enumerate() {
        match ast {
            // AstNode::Return(ast) => {
            //     ret = Some(eval(&ast, scope));
            //     break;
            // },
            _ => {
                let val = stmt(&ast, scope);
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


pub fn exec_if(cond_expr: &AstNode, body: &Vec<AstNode>, else_if: &Vec<AstNode>, else_body: &Vec<AstNode>, scope: &mut Scope) {
    if test_bool_val(eval(cond_expr, scope)) {
        stmt_body(body, scope);
        return;
    }
    for try_else_if in else_if {
        let (cond_expr, body) = match try_else_if {
            AstNode::ElseIf{ cond_expr, body } => (cond_expr, body),
            _ => panic!("expected ElseIf, got {:?}", try_else_if),
        };
        if test_bool_val(eval(cond_expr, scope)) {
            stmt_body(body, scope);
            return;
        }
    }
    stmt_body(else_body, scope);

}

pub fn stmt_body(body: &Vec<AstNode>, scope: &mut Scope) {
    for ast in body {
        stmt(ast, scope);
    }
}

pub fn stmt(ast: &AstNode, scope: &mut Scope) -> Value {
    match ast {
        // AstNode::FnDef{ name, args, body } => {
        //     scope.set_var(
        //         name,
        //         Value::Function(
        //             Function {
        //                 name: name.to_string(),
        //                 args: args.to_vec(),
        //                 body: body.to_vec(),
        //             }
        //         )
        //     );
        //     Value::None
        // },
        AstNode::VarDeclaration(Term::Ident(var), astbox) => {
            let val = eval(astbox, scope);
            scope.declare_var(var, val);
            Value::None
        },
        AstNode::Assignment(Term::Ident(var), astbox) => {
            let val = eval(astbox, scope);
            scope.set_var(var, val);
            Value::None
        },
        AstNode::If{ cond_expr, body, else_if, else_body } => {
            exec_if(cond_expr, body, else_if, else_body, scope);
            Value::None // todo
        }
        _ => eval(ast, scope),
    }
}

fn arithmetic(lhs: Value, op: Op, rhs: Value) -> Value {
    match lhs {
        Value::Integer(i) => arith_int(i, op, rhs),
        // Value::DoublePrecisionFloat(f) => arith_float(f, op, rhs),
        Value::Str(s) => arith_str(s, op, rhs),
        Value::Bool(_) => panic!("todo implement bool arith"), // arith_bool(s, op, rhs),
        Value::Function(_) => panic!("can't {:?} on function", op), // arith_bool(s, op, rhs),
        Value::List(_) => panic!("todo implement list arith"),
        // Value::Undefined => panic!("Can't {:?} undefined and {:?}", op, rhs),
        Value::None => panic!("Can't {:?} None and {:?}", op, rhs),
    }
}

fn arith_int(a: i32, op: Op, rhs: Value) -> Value {
    let res = match rhs {
        Value::Integer(b) => match op {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
            Op::Shr => a >> b,
            Op::Shl => a << b,
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
            Op::Mod => a % b,
            Op::Exp => a.pow(b as u32),
        },
        _ => panic!("Can't {:?} int {} with {:?}", op, a, rhs),
    };
    Value::Integer(res)
}

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

pub fn arith_str(a: String, op: Op, rhs: Value) -> Value {
    let res = match rhs {
        Value::Str(b) => match op {
            Op::Add => a + &b,
            _ => panic!("operator {:?} not defined for string", op),
        },
        _ => panic!("Can't {:?} string {} with {:?}", op, a, rhs),
    };
    Value::Str(res)
}

pub fn eval(ast: &AstNode, scope: &mut Scope) -> Value {
    match ast {
        AstNode::FnCall{ name, args } => fn_call(name, args, scope),
        AstNode::FnDef{ name, args, body} => Value::Function(
            Function { name: name.to_string(), args: args.to_vec(), body: body.to_vec() }
        ),
        AstNode::Arithmetic(lhs, op, rhs) => arithmetic(eval(lhs, scope), op.clone(), eval(rhs, scope)),
        AstNode::Term(Term::Str(x)) => Value::Str(x.to_string()),
        AstNode::Term(Term::Integer(x)) => Value::Integer(*x),
        AstNode::Term(Term::Bool(x)) => Value::Bool(*x),
        // AstNode::Term(Term::DoublePrecisionFloat(x)) => Value::DoublePrecisionFloat(*x),
        AstNode::Term(Term::Ident(var)) => {
            scope.get_var(var)
        }
        _ => panic!("Unexpected ast {:?}", ast),
    }
}

pub fn run(ast_list: Vec<AstNode>) {
    let mut global_scope = Scope::new(String::from("<top level>"));
    for ast_node in ast_list {
        stmt(&ast_node, &mut global_scope);
    }
}