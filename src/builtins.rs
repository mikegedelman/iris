use crate::run::Value;

fn fmt(val: &Value) -> String {
    match val {
        Value::Integer(x) => format!("{}", x),
        Value::Str(x) => format!("{}", x),
        Value::Bool(x) => format!("{}", x),
        Value::Function(f) => format!("{}", f.name),
        Value::List(vs) => {
            let strings: Vec<String> = vs.iter().map(|v| fmt(v)).collect();
            format!("[{}]", strings.join(", "))
        },
        Value::None => "None".to_string(),
        // Value::DoublePrecisionFloat(x) => format!("{}", x),
        // Value::Undefined => String::from("undefined"),
    }
}

pub fn print(args: Vec<Value>) -> Value {
    let print_strs: Vec<String> = args.iter().map(|arg| fmt(&arg)).collect();
    let joined = print_strs.join(" ");
    println!("{}", joined);
    Value::None
}