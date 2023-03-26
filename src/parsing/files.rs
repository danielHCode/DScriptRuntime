use std::fs::File;
use std::io::{BufReader, Read};
use std::process::id;
use std::str::FromStr;
use crate::{EqualityCheck, Operation, Type};
use crate::runtime::BinaryOpCode;

const FUNCTION_IDENTIFIER: u8 = 0;
const GLOBAL_VAR_IDENTIFIER: u8 = 1;
const TYPE_IDENTIFIER: u8 = 2;



fn open_binary_file(path: String) -> Vec<u8> {
    BufReader::new(File::open(path).unwrap()).buffer().iter().map(|it| it.clone()).collect()
}

fn validate_header(bytes: Vec<u8>) -> bool {
    String::from_utf8(bytes[0..7].to_owned()).unwrap() == "DSCRIPT"
}

fn open_text_file(path: String) -> String {
    let mut text = String::new();
    File::open(path).unwrap().read_to_string(&mut text).expect("Cannot read file");
    text
}

fn parse_type_args(str: &str) -> Type {
    match str {
        "std/str" => Type::Str,
        "std/num" => Type::Num,
        "std/bool" => Type::Bool,
        "std/any" => Type::Void,
        "std/list" => Type::List(Box::new(Type::Void)),
        _ => Type::Complex(vec![])
    }
}

fn parse_error(message: &str, i: usize) -> (String, usize) {
    (message.to_string(), i)
}

fn parse_instructions(mut i: usize, words: &Vec<String>) -> Result<(Vec<Operation>, usize), (String, usize)> {
    let mut instructions = vec![];

    loop {
        let word = words[i.clone()].to_owned();
        instructions.push(match word.as_str() {
            "end" => break,
            "load" => {
                let name = words[i+1].to_string();
                i+=1;
                Operation::LoadVar(name)
            }
            "set" => {
                let name = words[i+1].to_string();
                i+=1;
                Operation::SetVar(name)
            }
            "dup" => Operation::Dup,
            "setProp" => {
                let name = words[i+1].to_string();
                i+=1;
                Operation::SetProperty(name)
            }
            "getProp" => {
                let name = words[i+1].to_string();
                i+=1;
                Operation::GetProperty(name)
            }
            "return" => Operation::Return,
            "binary" => {
                let op = match words[i+1].as_str() {
                    "add" => BinaryOpCode::Add,
                    "sub" => BinaryOpCode::Sub,
                    "mul" => BinaryOpCode::Mul,
                    "div" => BinaryOpCode::Div,
                    _ => return Err(parse_error("Invalid Binary token", i))
                };
                i+=1;
                Operation::BinaryOp(op)
            },
            "equality" => {
                let op = match words[i+1].as_str() {
                    "eq" => EqualityCheck::Eq,
                    "neq" => EqualityCheck::Neq,
                    "gt" => EqualityCheck::Gt,
                    "st" => EqualityCheck::St,
                    _ => return Err(parse_error("Invalid Equality token token", i))
                };
                i+=1;
                Operation::EqualityCheck(op)
            },
            "loadNum" => {
                i+=1;
                match f64::from_str(words[i].as_str()) {
                    Ok(f) => Operation::LoadConstNum(f),
                    Err(_) => return Err(parse_error("Expected number after loadNum", i))
                }
            },
            "loadBool" => {
                i+=1;
                Operation::LoadConstBool(match words[i].as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err(parse_error("expected bool after loadBool", i))
                })
            },
            "loadString" => {
                todo!("")
            }
            "call" => {
                i+=1;
                let identifier = words[i].to_string();
                i+=1;
                match u32::from_str(words[i].as_str()) {
                    Ok(v) => Operation::CallFunction {signature: identifier, argc: v},
                    Err(_) => return Err(parse_error("Expected Argc after call signature", i))
                }
            },
            "mapArg" => {
                i+=1;
                let arg = match u32::from_str(words[i].as_str()) {
                    Ok(v) => v as usize,
                    Err(_) => return Err(parse_error("Expected arg index", i))
                };
                i+=1;
                let name = words[i].to_string();
                Operation::MapArgTo {arg, name}
            },
            "loadArg" => {
                i+=1;
                let arg = match u32::from_str(words[i].as_str()) {
                    Ok(v) => v as usize,
                    Err(_) => return Err(parse_error("Expected arg index", i))
                };
                Operation::LoadArg(arg)
            },
            "@List" => {
                i+=1;
                let arg = match u32::from_str(words[i].as_str()) {
                    Ok(v) => v,
                    Err(_) => return Err(parse_error("Expected arg index", i))
                };
                Operation::InitList {init_push: arg}
            },
            "@Object"
            _ => return Err(parse_error("Invalid Token", i))
        });
        i+=1;
    }

    Ok((instructions, i))
}

fn parse_sections(code: String) -> Result<(), String> {
    let mut i: usize = 0;
    let words = code.replace("\n", " ").split(" ").map(|it| it.to_owned()).collect::<Vec<String>>();
    while i < words.len() {
        match words[i].as_str() {
            "func" => {

                //signature
                let signature = words[i+1].clone();
                i+=1;

                //args
                let mut args = vec![];
                while words[i] != "endArgs" {
                    args.push(parse_type_args(words[i].as_str()));
                    i+=1;
                }

                //return type
                let return_type = parse_type_args(words[i+1].as_str());
                assert_eq!(words[i + 2], "do");
                i+=2;

                //instructions
                let instructions = parse_instructions(i, &words);
            }
            _ => {}
        }
    }

    Ok(())
}