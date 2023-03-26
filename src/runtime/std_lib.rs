use std::collections::HashMap;
use std::fmt::format;
use std::io::stdin;
use std::num::ParseFloatError;
use std::str::FromStr;
use std::time::SystemTime;
use crate::Operation;
use crate::runtime::{Function, Object, RuntimeObject, Type};
use crate::runtime::Operation::Native;
use crate::runtime::util::{dynamic_function, dynamic_library_function, function, library_function};

pub fn get_std_library() -> Vec<Function> {
    vec![
        dynamic_library_function(
            "std/io/print",
            |args, _| {
                for arg in args {
                    print!("{}", arg)
                }
                print!("\n");
                Ok(RuntimeObject::Void)
            },
            Type::Void
        ),
        library_function(
            "std/io/read",
            vec![],
            |_, _| {
                let mut buffer = String::new();
                let stdin = stdin(); // We get `Stdin` here.
                match stdin.read_line(&mut buffer) {
                    Ok(_) => Ok(RuntimeObject::Str(buffer)),
                    Err(_) => Err(format!("Error while reading std in"))
                }

            },
            Type::Str
        ),
        library_function(
            "*:as_string",
            vec![Type::Void],
            |args, _| {
                Ok(RuntimeObject::Str(format!("{}", args[0])))
            },
            Type::Str
        ),
        library_function(
            "str:as_number",
            vec![Type::Str],
            |args, _| {
                match args[0].clone() {
                    RuntimeObject::Str(s) => {
                        match f64::from_str(s.as_str()) {
                            Ok(x) => Ok(RuntimeObject::Num(x)),
                            Err(_) => Err(format!("Cannot convert string '{}' into a number", s))
                        }
                    }
                    _ => Err(format!("expected string on string function"))
                }
            },
            Type::Num
        ),
        library_function(
            "list:get",
            vec![Type::List(Box::new(Type::Void)), Type::Num],
            |args, _| {
                let list = match &args[0] {
                    RuntimeObject::List(v) => v,
                    _ => return Err(format!("sub-function of list must be called on list"))
                };

                let index: usize = match &args[1] {
                    RuntimeObject::Num(n) => { *n as usize },
                    _ => return Err(format!("expected index as arg of list:get"))
                };

                Ok(list[index].clone())
            },
            Type::Void
        ),
        library_function(
            "list:set",
            vec![Type::List(Box::new(Type::Void)), Type::Num, Type::Void],
            |args, _| {
                let mut list = match &args[0] {
                    RuntimeObject::List(v) => v,
                    _ => return Err(format!("sub-function of list must be called on list"))
                }.clone();

                let index: usize = match &args[1] {
                    RuntimeObject::Num(n) => { *n as usize },
                    _ => return Err(format!("expected index as arg of list:get"))
                };

                list[index] = args[2].clone();

                Ok(RuntimeObject::List(list))
            },
            Type::Void
        ),
        library_function(
            "list:add",
            vec![Type::List(Box::new(Type::Void)), Type::Void],
            |args, _| {
                let mut list = match &args[0] {
                    RuntimeObject::List(v) => v,
                    _ => return Err(format!("sub-function of list must be called on list"))
                }.clone();

                list.push(args[1].clone());

                Ok(RuntimeObject::List(list))
            },
            Type::Void
        )

    ]
}