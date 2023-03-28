use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;
use crate::{EqualityCheck, Operation, Type};
use crate::runtime::{BinaryOpCode, Function};


#[no_mangle]
fn open_binary_file(path: String) -> Vec<u8> {
    BufReader::new(File::open(path).unwrap()).buffer().iter().map(|it| it.clone()).collect()
}

#[no_mangle]
fn validate_header(bytes: Vec<u8>) -> bool {
    String::from_utf8(bytes[0..7].to_owned()).unwrap() == "DSCRIPT"
}

pub fn open_text_file(path: String) -> Result<String, String> {
    let mut text = String::new();
    match File::open(path){
        Ok(mut v) => {
            v.read_to_string(&mut text).expect("Cannot read file");
        },
        Err(_) => return Err("cannot open file".to_string())
    };
    Ok(text)
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
                i+=1;
                let mut str = String::new();
                while !words[i].ends_with("'") {
                    str+=words[i].as_str();
                    i+=1;
                }
                str=format!("{} {}", str, words[i].to_string().strip_suffix("'").unwrap());

                Operation::LoadConstString(str)
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
            "@Object" => {
                let mut names = vec![];
                loop {
                    i+=1;
                    let word2 = words[i].clone();
                    if word2 == "#" {
                        break;
                    }
                    names.push(word2);
                }
                Operation::InitObject { keys: names, template: None }
            },
            "if" => {
                match parse_scope(i, words) {
                    Ok((ins, j)) => {
                        i=j;
                        Operation::If(ins)
                    }
                    Err(e) => return Err(e)
                }
            },
            "else" => {
                match parse_scope(i, words) {
                    Ok((ins, j)) => {
                        i=j;
                        Operation::Else(ins)
                    }
                    Err(e) => return Err(e)
                }
            },
            "while" => {
                match parse_scope(i, words) {
                    Ok((cond, j)) => {
                        i=j;
                        match parse_scope(i, words) {
                            Ok((content, j)) => {
                                i=j;
                                Operation::While { condition: cond, content: content }
                            }
                            Err(e) => return Err(e)
                        }
                    }
                    Err(e) => return Err(e)
                }
            }
            _ => return Err(parse_error("Invalid Token (inner)", i))
        });
        i+=1;
    }

    Ok((instructions, i))
}

fn parse_scope(mut i: usize, words: &Vec<String>) -> Result<(Vec<Operation>, usize), (String, usize)> {
    i+=1;
    assert_eq!(words[i].to_string(), "do");
    i+=1;
    match parse_instructions(i, words) {
        Ok((ins, j)) => Ok((ins, j)),
        Err(e) => Err((e.0.to_owned(), e.1.clone()))
    }
}

pub fn parse_sections(words: &Vec<String>) -> Result<Vec<Function>, (String, usize)> {
    let mut i: usize = 0;
    let mut functions = vec![];
    while i < words.len() {
        match words[i].as_str() {
            "func" => {

                //signature
                let signature = words[i+1].clone();
                i+=2;

                //args
                let mut args = vec![];
                while words[i] != "endArgs" {
                    args.push(parse_type_args(words[i].as_str()));
                    i+=1;
                }

                i+=1;

                //return type
                let return_type = parse_type_args(words[i+1].as_str());
                i+=1;

                //instructions
                match parse_instructions(i, &words) {
                    Ok((instructions, j)) => {
                        i=j;
                        functions.push(Function {signature,args: Some(args), instructions, return_type })
                    }
                    Err(e) => return Err(e)
                }

            }
            _ => return Err(parse_error("Invalid token outside of function)", i))
        }
        i+=1;
    }

    Ok(functions)
}