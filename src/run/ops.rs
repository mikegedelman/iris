/// Unary and binary operators implementations

use crate::ast::Op;
use super::Value;

pub fn infix(lhs: Value, op: Op, rhs: Value) -> Value {
    match lhs {
        Value::Integer(i) => infix_int(i, op, rhs),
        // Value::DoublePrecisionFloat(f) => arith_float(f, op, rhs),
        Value::Str(s) => infix_str(s, op, rhs),
        Value::Bool(_) => panic!("todo implement bool arith"), // arith_bool(s, op, rhs),
        Value::Function(_) => panic!("can't {:?} on function", op), // arith_bool(s, op, rhs),
        Value::List(l) => infix_list(l, op, rhs),
        // Value::Undefined => panic!("Can't {:?} undefined and {:?}", op, rhs),
        Value::None => panic!("Can't {:?} None and {:?}", op, rhs),
        _ => panic!("unimplemented infix for {:?}", lhs),
    }
}

pub fn unary(op: Op, rhs: Value) -> Value {
    Value::None
}

fn infix_int(a: i32, op: Op, rhs: Value) -> Value {
    match rhs {
        Value::Integer(b) => match op {
            Op::Add => Value::Integer(a + b),
            Op::Sub => Value::Integer(a - b),
            Op::Mul => Value::Integer(a * b),
            Op::Div => Value::Integer(a / b),
            Op::Shr => Value::Integer(a >> b),
            Op::Shl => Value::Integer(a << b),
            Op::And => Value::Integer(a & b),
            Op::Or => Value::Integer(a | b),
            // Op::Xor => Value::Integer(a ^ b),
            Op::Mod => Value::Integer(a % b),
            Op::Exp => Value::Integer(a.pow(b as u32)),
            // bools
            Op::GreaterThan => Value::Bool(a > b),
            Op::LessThan => Value::Bool(a < b),
            Op::Equal => Value::Bool(a == b),
            _ => panic!("{:?} not implemented", op),
        },
        _ => panic!("Can't {:?} int {} with {:?}", op, a, rhs),
    }
}

fn infix_list(a: Vec<Value>, op: Op, r: Value) -> Value {
    match r {
        Value::List(_) => match op {
            _ => panic!("{:?} not implemented for list, list", op),
        },
        Value::Integer(b) => match op {
            Op::MemberAccess => {
                a.get(b as usize).expect(&format!("index {} is out of bounds", b)).clone()
            },
            _ => panic!("{:?} not implemented for (list, int)", op),
        }
        _ => panic!("Can't {:?} list with {:?}", op, r),
    }
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

fn infix_str(a: String, op: Op, rhs: Value) -> Value {
    let res = match rhs {
        Value::Str(b) => match op {
            Op::Add => a + &b,
            _ => panic!("operator {:?} not defined for string", op),
        },
        _ => panic!("Can't {:?} string {} with {:?}", op, a, rhs),
    };
    Value::Str(res)
}