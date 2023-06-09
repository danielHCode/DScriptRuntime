pub(crate) mod std_lib;
mod object_storage;
mod util;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Pointer};
use crate::runtime::object_storage::ObjectStorage;
use crate::runtime::std_lib::get_std_library;

pub enum BinaryOpCode {
    Add,
    Sub,
    Mul,
    Div
}


pub enum EqualityCheck {
    Eq,
    Neq,
    Gt,
    St
}

pub enum Operation {
    LoadConstNum(f64),      //d
    LoadConstString(String),//TODO
    LoadConstBool(bool),    //d
    CallFunction {signature: String, argc: u32},    //D
    Return,                 //d
    Dup,                    //d
    BinaryOp(BinaryOpCode), //d
    EqualityCheck(EqualityCheck),   //d
    Native { callback: fn(&Vec<RuntimeObject>, &mut ObjectStorage) -> Result<RuntimeObject, String> },
    If(Vec<Operation>),     //d
    Else(Vec<Operation>),   //d
    While { condition: Vec<Operation>, content: Vec<Operation> },             //d
    InitObject { keys: Vec<String>, template: Option<HashMap<String, Type>> },//d
    InitList { init_push: u32 },                                              //d
    SetProperty(String),    //d
    GetProperty(String),    //d
    SetVar(String),         //d
    LoadVar(String),        //d
    MapArgTo {arg: usize, name: String},    //d
    LoadArg(usize),         //d
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::LoadConstNum(n) => f.write_fmt(format_args!("LoadConstNum({})", n)),
            Operation::LoadConstString(_) => f.write_str("LoadConstString"),
            Operation::LoadConstBool(_) => f.write_str("LoadConstBool"),
            Operation::CallFunction { signature, argc } => f.write_fmt(format_args!("FunctionCall({}, {})", signature, argc)),
            Operation::Return => f.write_str("Return"),
            Operation::Dup => f.write_str("Dup"),
            Operation::BinaryOp(_) => f.write_str("Binary"),
            Operation::EqualityCheck(_) => f.write_str("EqualityCheck"),
            Operation::Native { callback: _callback } => f.write_str("Native"),
            Operation::If(content) => f.write_fmt(format_args!("IfDo({})", content.iter().fold(String::new(), |a, b| format!("{}, {}", a, b)))),
            Operation::Else(content) => f.write_fmt(format_args!("ElseDo({})", content.iter().fold(String::new(), |a, b| format!("{}, {}", a, b)))),
            Operation::While { condition, content } => f.write_fmt(
                format_args!(
                    "While({})do({})", 
                    condition.iter().fold(String::new(), |first, a| format!("{}, {}", first, a)), 
                    content.iter().fold(String::new(), |first, a| format!("{}, {}", first, a))
                )),
            Operation::InitObject { keys: _, template: _ } => f.write_fmt(format_args!("InitObject")),
            Operation::InitList { init_push } => f.write_fmt(format_args!("List({})", init_push)),
            Operation::SetProperty(s) => f.write_fmt(format_args!("SetProperty({})", s)),
            Operation::GetProperty(s) => f.write_fmt(format_args!("GetProperty({})", s)),
            Operation::SetVar(name) =>  f.write_fmt(format_args!("SetVar({})", name)),
            Operation::LoadVar(name) => f.write_fmt(format_args!("LoadVar({})", name)),
            Operation::MapArgTo { arg, name } => f.write_fmt(format_args!("MapArg({} to {})", arg, name)),
            Operation::LoadArg(_) => f.write_str("LoadArg"),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Type {
    Num,
    Str,
    Bool,
    List(Box<Type>),
    Complex(Vec<Type>),
    Void
}

impl Type {
    fn to_string(&self) -> String {
        match self {
            Type::Num => "Number".to_string(),
            Type::Str => "String".to_string(),
            Type::Bool => "Boolean".to_string(),
            Type::List(tp) => format!("List<{}>", tp.as_ref().to_string()),
            Type::Complex(_) => "Complex".to_string(),
            Type::Void => "Void".to_string()
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Num => f.write_str("Number"),
            Type::Str => f.write_str("String"),
            Type::Bool => f.write_str("Boolean"),
            Type::List(tp) => {
                f.write_str("List<").unwrap();
                tp.as_ref().fmt(f).unwrap();
                f.write_str(">")
            },
            Type::Complex(_) => {
                f.write_str("Complex")
            }
            Type::Void => f.write_str("Void")
        }
    }
}

pub struct Function {
    pub(crate) signature: String,
    pub(crate) args: Option<Vec<Type>>,
    pub(crate) instructions: Vec<Operation>,
    pub(crate) return_type: Type
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature && self.args == other.args && self.instructions.len() == other.instructions.len() && self.return_type == other.return_type
    }
}

/*

Object definition:

 */

#[derive(Clone)]
pub struct Object {
    id: usize,
    //values: HashMap<String, RuntimeObject>,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Object {

    fn get_signature(&self) -> String {
        format!(
            "Object<!{}>",
            /*self.values.iter().fold(
                String::new(),
                |str, (str2, rto)| format!("{}, {}: {}", str, str2, rto.get_type().to_string())
            ).as_str()
             */
            self.id
        )
    }
}

/*

Runtime Object

 */

#[derive(PartialEq)]
pub enum RuntimeObject {
    Object(Object),
    //Function(Function),
    Num(f64),
    List(Vec<RuntimeObject>),
    Str(String),
    Bool(bool),
    Void
}

impl Display for RuntimeObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeObject::Num(n) => f.write_str(n.to_string().as_str()),
            RuntimeObject::Bool(b) => f.write_str(b.to_string().as_str()),
            RuntimeObject::Str(s) => f.write_str(s.as_str()),
            RuntimeObject::Void => f.write_str("Void"),
            RuntimeObject::List(vec) => {
                f.write_str("[")?;
                vec.fmt(f)?;
                f.write_str("]")
            }
            RuntimeObject::Object(o) => {
                o.fmt(f)
            }
            //RuntimeObject::Function(_) => f.write_str("Function(="),
        }
    }
}


impl Clone for RuntimeObject {
    fn clone(&self) -> Self {
        match self {
            RuntimeObject::Num(num) => RuntimeObject::Num(num.clone()),
            RuntimeObject::Str(str) => RuntimeObject::Str(str.clone()),
            RuntimeObject::Bool(bool) => RuntimeObject::Bool(bool.clone()),
            RuntimeObject::Void => RuntimeObject::Void,
            RuntimeObject::Object(o) => RuntimeObject::Object(o.clone()),
            RuntimeObject::List(l) => RuntimeObject::List(l.iter().map(|it| it.clone()).collect())
            //RuntimeObject::Function(f) => RuntimeObject::Function(f.clone())
        }
    }
}


impl RuntimeObject {
    fn get_type(&self) -> Type {
        match self {
            RuntimeObject::Object(_) => {
                Type::Complex(vec![])
            }
            RuntimeObject::Num(_) => Type::Num,
            RuntimeObject::Str(_) => Type::Str,
            RuntimeObject::Bool(_) => Type::Bool,
            RuntimeObject::Void => Type::Void,
            RuntimeObject::List(_) => Type::List(Box::new(Type::Void))
        }
    }
}


fn map_objects_to_type(objects: &Vec<RuntimeObject>) -> Vec<Type> {
    objects.iter().map(|it| it.get_type()).collect()
}

fn compare_types(received: &Vec<Type>, expected: &Vec<Type>, context_sig: &str) -> Result<(), String> {
    if received.len() != expected.len() {
        return Err(format!("Expected {} arguments but got {} args while trying to call {}!", expected.len(), received.len(), context_sig));
    }

    for (i, tp) in expected.iter().enumerate() {
        if *tp == Type::Void {
            continue
        }
        if *tp != received[i] {
            return Err(format!("Expected th {} Arg of type {} while calling {} but received type {}!",i, tp, context_sig, received[i]))
        }
    }

    Ok(())
}

fn binary_operation(first: &RuntimeObject, second: &RuntimeObject, op: &BinaryOpCode) -> Result<RuntimeObject, String> {
    if first.get_type() != second.get_type() {
        return Err(format!("Binary Operations can only be executed on the same type... got {} and {}", first.get_type().to_string(), second.get_type().to_string()))
    }

    match first {
        RuntimeObject::Num(num) => {
            match second {
                RuntimeObject::Num(num2) => Ok(RuntimeObject::Num(match op {
                    BinaryOpCode::Add => num + num2,
                    BinaryOpCode::Sub => num - num2,
                    BinaryOpCode::Mul => num * num2,
                    BinaryOpCode::Div => num / num2
                })),
                obj => Err(format!("Cannot operate {} on Number while doing binary operations", obj.get_type()))
            }
        }
        RuntimeObject::Str(str) => {
            match op {
                BinaryOpCode::Add => {
                    Ok(RuntimeObject::Str(match second {
                        RuntimeObject::Str(value) => str.to_owned()+value.as_str(),
                        RuntimeObject::Num(num) => str.to_owned()+num.to_string().as_str(),
                        RuntimeObject::Bool(bool) => str.to_owned()+bool.to_string().as_str(),
                        RuntimeObject::Void => str.to_owned()+"Void",
                        RuntimeObject::Object(o) => str.to_owned()+o.get_signature().as_str(),
                        RuntimeObject::List(_) => "List<>".to_string()
                    }))
                }
                _ => Err(format!("Doing Binary Operations other than add on String does not make sense"))
            }
        }
        e => Err(format!("Cannot do binary Operation on type {}", e.get_type()))
    }
}


fn equality_check(first: &RuntimeObject, second: &RuntimeObject, op: &EqualityCheck) -> Result<RuntimeObject, String> {
    match op {
        EqualityCheck::Eq => Ok(RuntimeObject::Bool(*first == *second)),
        EqualityCheck::Neq => Ok(RuntimeObject::Bool(*first != *second)),
        EqualityCheck::Gt => match first {
            RuntimeObject::Num(num) => {
                match second {
                    RuntimeObject::Num(num2) => Ok(RuntimeObject::Bool(num2 > num)),
                    _ => Err(format!("Invalid"))
                }
            }
            _ => Err(format!("invalid"))
        }
        EqualityCheck::St => match first {
            RuntimeObject::Num(num) => {
                match second {
                    RuntimeObject::Num(num2) => Ok(RuntimeObject::Bool(num2 < num)),
                    _ => Err(format!("Invalid"))
                }
            }
            _ => Err(format!("invalid"))
        }
    }
}

pub fn execute_std(mut functions: Vec<Function>, execution_signature: &str) {
    let mut std = get_std_library();
    std.append(&mut functions);

    let mut runtime = Runtime { storage: ObjectStorage::new() };

    match runtime.execute(&std, execution_signature, vec![]) {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

}

fn dec_all(storage: &mut ObjectStorage, objects: &Vec<&RuntimeObject>) {
    for obj in objects {
        match obj {
            RuntimeObject::Object(o) => storage.dec_reference_count(o),
            _ => {}
        }
    }
}

pub struct Runtime {
    pub(crate) storage: ObjectStorage
}

impl Runtime {

    pub fn new() -> Runtime {
        Runtime { storage: ObjectStorage { object_storage: vec![], allocation_table: HashMap::new() }}
    }

    pub fn execute(&mut self, functions: &Vec<Function>, execution_signature: &str, args: Vec<RuntimeObject>) -> Result<RuntimeObject, String> {
        let function = functions
            .iter()
            .find(|it| it.signature==execution_signature)
            .unwrap();
        let mut variables: HashMap<String, RuntimeObject> = HashMap::new();
        return self.execute_function(functions, &function.instructions, execution_signature, &args, &mut variables)
    }

    fn execute_function(
        &mut self,
        functions: &Vec<Function>,
        instructions: &Vec<Operation>,
        execution_signature: &str,
        args: &Vec<RuntimeObject>,
        variables: &mut HashMap<String, RuntimeObject>
    ) -> Result<RuntimeObject, String> {

        let mut stack: Vec<RuntimeObject> = vec![];

        for (_, instruction) in instructions.iter().enumerate() {
            match instruction {
                //load constants operation
                Operation::LoadConstNum(num) => stack.push(RuntimeObject::Num(num.to_owned())),
                Operation::LoadConstString(str) => stack.push(RuntimeObject::Str(str.to_owned())),
                Operation::LoadConstBool(bool) => stack.push(RuntimeObject::Bool(bool.to_owned())),

                //function calls
                Operation::CallFunction { signature, argc } => {
                    let mut args = vec![];
                    for _ in 0..argc.to_owned() {
                        args.push(match stack.pop().unwrap() {
                            RuntimeObject::Object(o) => {
                                self.storage.inc_reference_count(&o);
                                RuntimeObject::Object(o)
                            },
                            obj => obj
                        })
                    }

                    let function = match functions.iter().find(|it| it.signature == *signature) {
                        Some(t) => t,
                        None => return Err(format!("No function found with name {}, while executing {}", signature, execution_signature))
                    };

                    if function.args.is_some() {
                        match compare_types(
                            &map_objects_to_type(&args),
                            &function.args.as_ref().unwrap(),
                            signature.as_str()
                        ) {
                            Ok(..) => {  }
                            Err(e) => return Err(format!("{}\n Error occurred in {}", e, execution_signature))
                        }
                    }


                    match self.execute(functions, signature.as_str(), args) {
                        Ok(result) => stack.push(result),
                        Err(e) => return Err(format!("{}\n Error occurred in {}", e, execution_signature))
                    }
                }

                Operation::BinaryOp(op) => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();

                    match binary_operation(&first, &second, &op) {
                        Ok(result) => stack.push(result),
                        Err(e) => return Err(format!("{}\n Error while executing {}", e, execution_signature))
                    }
                }

                Operation::EqualityCheck(op) => {
                    let first = stack.pop().unwrap();
                    let second = stack.pop().unwrap();

                    match equality_check(&first, &second, &op) {
                        Ok(result) => stack.push(result),
                        Err(r) => return Err(format!("{}\n Error while executing {}", r, execution_signature))
                    }
                }

                Operation::Native {callback} => {
                    match callback(&args, &mut self.storage) {
                        Ok(result) => stack.push(result),
                        Err(e) => return Err(format!("{}\n Error while executing {}", e, execution_signature))
                    }
                }

                Operation::Return => {
                    let return_value = match stack.pop().unwrap() {
                        RuntimeObject::Object(o) => {
                            self.storage.inc_reference_count(&o);
                            RuntimeObject::Object(o)
                        },
                        obj => obj
                    };
                    dec_all(&mut self.storage, &variables.values().collect());
                    dec_all(&mut self.storage, &stack.iter().collect());
                    return Ok(return_value)
                }
                Operation::If(content) => {
                    match stack.pop().unwrap() {
                        RuntimeObject::Bool(val) => {
                            if val {
                                match self.execute_function(functions, content, execution_signature, args, variables) {
                                    Ok(result) => stack.push(result),
                                    Err(e) => return Err(format!("{}\n Error while executing {}", e, execution_signature))
                                }
                            }
                            stack.push(RuntimeObject::Bool(val))
                        }
                        _ => {return Err(format!("Expected Bool on stack (If) while executing {}", execution_signature))}
                    }
                }
                Operation::Else(content) => {
                    match stack.pop().unwrap() {
                        RuntimeObject::Bool(val) => {
                            if !val {
                                match self.execute_function(functions, content, execution_signature, args, variables) {
                                    Ok(result) => stack.push(result),
                                    Err(e) => return Err(format!("{}\n Error while executing {}", e, execution_signature))
                                }
                            }
                            stack.push(RuntimeObject::Bool(val))
                        }
                        _ => {return Err(format!("Expected Bool on stack (Else) while executing {}", execution_signature))}
                    }
                }
                Operation::While {content, condition} => {

                    while match self.execute_function(functions, condition, execution_signature, args, variables) {
                        Ok(result) => match result {
                            RuntimeObject::Bool(val) => val,
                            _ => return Err(format!("Expected boolean in while condition while executing {}", execution_signature))
                        },
                        Err(e) => return Err(format!("{}\n Error while executing {}", e, execution_signature))
                    } {
                        match self.execute_function(functions, content, execution_signature, args, variables) {
                            Ok(result) => match result {
                                RuntimeObject::Bool(val) => {if val {
                                    break
                                }}
                                _ => {}
                            },
                            Err(e) => return Err(format!("{}\n Error while executing {}", e, execution_signature))
                        }
                    }
                }
                Operation::SetVar(name) => {
                    let data = stack.pop().unwrap();
                    variables.insert(name.to_string(), data);                }
                Operation::LoadVar(name) => {
                    let var = variables[name.as_str()].clone();
                    stack.push(var);
                }
                Operation::Dup => {
                    let var = stack.pop().unwrap();
                    stack.push(var.clone());
                    stack.push(var)
                }

                Operation::InitObject {keys, template} => {
                    let mut error_flag = false;
                    let object = self.storage.allocate_object();
                    self.storage.replace_fields(&object, keys.iter().map(|key| {
                        let value = stack.pop().unwrap();
                        if template.is_some() {
                            if !template.as_ref().unwrap().contains_key(key) || value.get_type() != *template.as_ref().unwrap().get(key).unwrap().clone(){
                                error_flag = true;
                            }
                        }
                        (key.to_string(), value)
                    }).collect());

                    if error_flag {
                        return Err(format!("Invalid types while object creation {}", execution_signature))
                    }

                    stack.push(RuntimeObject::Object(object))
                }

                Operation::InitList { init_push } => {
                    let values: Vec<RuntimeObject> = (0..*init_push).into_iter().map(|_| stack.pop().expect(&*format!("Expected {} elements on stack for list init while executing {}", init_push, execution_signature))).collect();
                    stack.push(RuntimeObject::List(values))
                }

                Operation::SetProperty(name) => {
                    match stack.pop().unwrap() {
                        RuntimeObject::Object(o) => {
                            let item = stack.pop().unwrap();
                            match &item {
                                RuntimeObject::Object(i) => self.storage.inc_reference_count(&i),
                                _ => {}
                            }
                            self.storage.set_field(&o, name.to_string(), item);
                            stack.push(RuntimeObject::Object(o));
                        }
                        e => return Err(format!("Expected Object on the stack for set property op, while executing {}, {}", execution_signature, e))
                    };
                }

                Operation::GetProperty(name) => {
                    match stack.pop().expect("Expected Value on stack while calling getProperty") {
                        RuntimeObject::Object(o) => {
                            let item = self.storage.get_field(&o, name.to_string()).expect(format!("Property {} does not exist on object\n while executing {}", name, execution_signature).as_str());
                            stack.push(item.clone())
                        }
                        _ => return Err(format!("Expected Object on the stack for set property op, while executing {}", execution_signature))
                    };
                }

                Operation::MapArgTo {arg, name} => {
                    variables.insert(name.to_string(), args[arg.clone()].clone());
                }

                Operation::LoadArg(arg) => {
                    stack.push(args[arg.clone()].clone())
                }
            };
        }

        Err(format!("Function did not return but ran out of tokens ({})", execution_signature))
    }
}


