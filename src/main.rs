extern crate core;

use crate::Operation::While;
use crate::runtime::{EqualityCheck, execute_std, Function, Operation, Type};
use crate::runtime::BinaryOpCode::{Add};


mod runtime;
mod parsing;

fn main() {
    let x = "x".to_string();

    execute_std(vec![
        Function {
            signature: "main/main".to_string(),
            args: Some(vec![]),
            instructions: vec![
                /*
                Operation::LoadConstNum(1.0),
                Operation::SetVar(x.to_string()),
                While {condition: vec![
                    Operation::LoadConstNum(1000000.0),
                    Operation::LoadVar(x.to_string()),
                    Operation::EqualityCheck(EqualityCheck::Gt),
                    Operation::Return
                ], content: vec![
                    Operation::LoadVar(x.to_string()),
                    Operation::LoadConstNum(1.0),
                    Operation::BinaryOp(Add),
                    Operation::SetVar(x.to_string()),
                ]},
                Operation::LoadVar(x.to_string()),
                */
                Operation::LoadConstNum(679.9),
                Operation::LoadConstNum(679.9),
                Operation::LoadConstNum(679.9),

                Operation::CallFunction {signature: "*:as_string".to_string(), argc: 1},
                Operation::CallFunction {signature: "std/io/print".to_string(), argc: 1},
            ],
            return_type: Type::Void
        }
    ], "main/main")
}


