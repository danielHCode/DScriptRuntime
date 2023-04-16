use std::str::FromStr;
use crate::runtime::std_lib::io_functions::{io_print, io_read, io_open_file, file_read_to_string, file_write_string};
use crate::runtime::std_lib::str_functions::{str_split, str_replace, str_to_lower, str_to_upper};
use crate::runtime::{Function, RuntimeObject, Type};
use crate::runtime::util::{library_function, dynamic_library_function};

use super::Object;

mod io_functions;
mod str_functions;
mod result;

fn assert_arg_length(args: &Vec<RuntimeObject>, size: usize) -> Result<(), String> {
    if args.len() == size {
        Ok(())
    } else {
        Err(format!("Expected {} arg but got {}", size, args.len()))
    }
}

fn get_as_string(args: &Vec<RuntimeObject>, index: usize) -> Result<String, String>{
    match &args[0] {
        RuntimeObject::Str(s) => Ok(s.to_string()),
        _ => Err(format!("Expected String at arg {}", index))
    }
}

fn get_as_object(args: &Vec<RuntimeObject>, index:usize) -> Result<Object, String> {
    match &args[0] {
        RuntimeObject::Object(o) => Ok(o.clone()),
        _ => Err(format!("Expected String at arg {}", index))
    }
}

pub fn get_std_library() -> Vec<Function> {
    vec![
        //io functions
        dynamic_library_function(
            "std/io/print",
            io_print,
            Type::Void
        ),
        library_function(
            "std/io/read",
            vec![],
            io_read,
            Type::Str
        ),
        library_function(
            "std/io/open_file",
            vec![Type::Str],
            io_open_file,
            Type::Complex(vec![])
        ),
        library_function(
            "file:read_to_string",
            vec![Type::Complex(vec![])],
            file_read_to_string,
            Type::Str
        ),
        library_function(
            "file:write_string",
            vec![Type::Complex(vec![]), Type::Str],
            file_write_string,
            Type::Void
        ),
        
        //string functions
        library_function(
            "*:as_string",
            vec![Type::Void],
            |args, _| {
                Ok(RuntimeObject::Str(format!("{}", args[0])))
            },
            Type::Str
        ),
        library_function(
            "*:replace",
            vec![Type::Str, Type::Str, Type::Str],
            str_replace,
            Type::Str
        ),
        library_function(
            "str:to_lower",
            vec![Type::Str],
            str_to_lower,
            Type::Str
        ),
        library_function(
            "str:to_upper",
            vec![Type::Str],
            str_to_upper,
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
            "str:split",
            vec![Type::Str, Type::Str],
            str_split,
            Type::List(Box::new(Type::Void))
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