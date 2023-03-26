use crate::{Function, Operation, Type};
use crate::Operation::{Native, Return};
use crate::runtime::object_storage::ObjectStorage;
use crate::runtime::RuntimeObject;

fn load_var(name: &str) -> Operation {
    Operation::LoadVar(name.to_string())
}

fn set_var(name: &str) -> Operation {
    Operation::SetVar(name.to_string())
}

pub fn function(identifier: &str, args: Vec<Type>, instructions: Vec<Operation>, return_type: Type) -> Function {
    Function {signature: identifier.to_string(), args: Some(args), instructions, return_type }
}

pub fn dynamic_function(identifier: &str, instructions: Vec<Operation>, return_type: Type) -> Function {
    Function {signature: identifier.to_string(), args: None, instructions, return_type }
}

pub fn library_function(identifier: &str, args: Vec<Type>, consumer: fn(&Vec<RuntimeObject>, &mut ObjectStorage) -> Result<RuntimeObject, String>, return_type: Type) -> Function {
    Function {
        signature: identifier.to_string(),
        args: Some(args),
        instructions: vec![
            Native {callback: consumer},
            Return
        ],
        return_type
    }
}

pub fn dynamic_library_function( identifier: &str, consumer: fn(&Vec<RuntimeObject>, &mut ObjectStorage) -> Result<RuntimeObject, String>, return_type: Type) -> Function {
    Function {
        signature: identifier.to_string(),
        args: None,
        instructions: vec![
            Native {callback: consumer},
            Return
        ],
        return_type
    }
}