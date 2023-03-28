use crate::runtime::{RuntimeObject, object_storage::ObjectStorage};

use super::{assert_arg_length, get_as_string};




pub fn str_split(args: &Vec<RuntimeObject>, _: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    assert_arg_length(args, 2)?;
    let base_string = get_as_string(args, 0)?;
    let split_string = get_as_string(args, 1)?;

    Ok(RuntimeObject::List(
        base_string
        .split(split_string.as_str())
        .map(|it| RuntimeObject::Str(it.to_string()))
        .collect::<Vec<RuntimeObject>>()
    ))
}

pub fn str_replace(args: &Vec<RuntimeObject>, _: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    assert_arg_length(args, 3)?;
    let base_string = get_as_string(args, 0)?;
    let target = get_as_string(args, 1)?;
    let replacement = get_as_string(args, 2)?;

    Ok(RuntimeObject::Str(base_string.replace(&target, &replacement)))
}

pub fn str_to_upper(args: &Vec<RuntimeObject>, _: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    assert_arg_length(args, 1)?;
    let base_string = get_as_string(args, 0)?;

    Ok(RuntimeObject::Str(base_string.to_uppercase()))
}

pub fn str_to_lower(args: &Vec<RuntimeObject>, _: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    assert_arg_length(args, 1)?;
    let base_string = get_as_string(args, 0)?;

    Ok(RuntimeObject::Str(base_string.to_lowercase()))
}