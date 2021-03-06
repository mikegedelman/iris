use crate::run::Value;

/// Format the given value into a string
fn fmt(val: &Value) -> String {
    match val {
        Value::Integer(x) => format!("{}", x),
        Value::Str(x) => format!("{}", x),
        Value::Bool(x) => format!("{}", x),
        Value::Function(f) => format!("function \"{}\"", f.name),
        Value::List(vs) => {
            let strings: Vec<String> = vs.iter().map(|v| fmt(v)).collect();
            format!("[{}]", strings.join(", "))
        },
        Value::None => "None".to_string(),
        Value::Some(s) => fmt(s),
    }
}

/// Builtin "print" function
pub fn print(args: Vec<Value>) -> Value {
    let print_strs: Vec<String> = args.iter().map(|arg| fmt(&arg)).collect();
    let joined = print_strs.join(" ");
    println!("{}", joined);
    Value::None
}

/// Builtin len() for collections
pub fn len(args: Vec<Value>) -> Value {
    assert_eq!(args.len(), 1, "len() accepts exactly one argument");
    match &args[0] {
        Value::List(xs) => Value::Integer(xs.len() as i32),
        _ => panic!("Can't get len() of a {:?}", args[0]),
    }
}


/// This and the following function provide an interace to work with an
/// Option type, like in rust. These are roughed in like this at the moment
/// to support iterators.
pub fn some(args: Vec<Value>) -> Value {
    assert_eq!(args.len(), 1, "Some() accepts exactly one argument");
    Value::Some(Box::new(args[0].clone()))
}

pub fn unwrap(args: Vec<Value>) -> Value {
    assert_eq!(args.len(), 1, "unwrap() accepts exactly one argument");
    let val = args[0].clone();
    match val {
        Value::Some(val) => *val,
        _ => panic!("Tried to unwrap() {:?}", val),
    }
}

pub fn is_some(args: Vec<Value>) -> Value {
    assert_eq!(args.len(), 1, "is_some() accepts exactly one argument");
    let val = &args[0];
    match val {
        Value::Some(_) => Value::Bool(true),
        Value::None => Value::Bool(false),
        _ => panic!("called is_some() on {:?}", val)
    }
}

pub fn is_none(args: Vec<Value>) -> Value {
    assert_eq!(args.len(), 1, "is_none() accepts exactly one argument");
    let val = &args[0];
    match val {
        Value::None => Value::Bool(true),
        Value::Some(_) => Value::Bool(false),
        _ => panic!("called is_none() on {:?}", val)
    }
}