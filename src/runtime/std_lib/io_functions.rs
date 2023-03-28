use std::{io::{stdin, Read, Write}, fs::File};

use crate::runtime::{RuntimeObject, object_storage::ObjectStorage};

use super::{get_as_string, get_as_object};


pub fn io_print(args: &Vec<RuntimeObject>, storage: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    for arg in args {
        println!("{}", d_format(arg, storage))
    }
    Ok(RuntimeObject::Void)
}

pub fn io_read(_: &Vec<RuntimeObject>, _: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("");
    Ok(RuntimeObject::Str(buffer))
}

pub fn io_open_file(args: &Vec<RuntimeObject>, storage: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    let path = get_as_string(args, 0)?;
    let file = match File::open(&path) {
        Err(_) => return Err(format!("file '{}' does not exist", &path)),
        Ok(f) =>f
    };

    let data = match file.metadata() {
        Ok(d) => d,
        Err(_) => return Err(format!("Cannot access metadata of file {}", &path))
    };

    let file_object = storage.allocate_object();

    for (key, value) in [
            ("path".to_string(), RuntimeObject::Str(path)),
            ("isFile".to_string(), RuntimeObject::Bool(data.is_file())),
            ("isDir".to_string(), RuntimeObject::Bool(data.is_dir())),
            ("length".to_string(), RuntimeObject::Num(data.len() as f64)),
        ] {
        storage.set_field(&file_object, key, value);
    };


    Ok(RuntimeObject::Object(file_object))
}

pub fn file_read_to_string(args: &Vec<RuntimeObject>, storage: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    let file_obj = get_as_object(args, 0)?;
    let path = match storage.get_field(&file_obj, "path".to_string()) {
        Some(f) => match f {
            RuntimeObject::Str(s) => s,
            _ => return Err(format!("Object Error Path must be string"))
        }
        None => return Err(format!("File Object must contain "))
    };

    let mut buf = String::new();

    match File::open(path) {
        Ok( mut f) => {
            f.read_to_string(&mut buf).expect("Cannot read file");
        }
        Err(_) => return Err(format!("File cannot be read"))
    };

    Ok(RuntimeObject::Str(buf))
}

pub fn file_write_string(args: &Vec<RuntimeObject>, storage: &mut ObjectStorage) -> Result<RuntimeObject, String> {
    let file_obj = get_as_object(args, 0)?;
    let path = match storage.get_field(&file_obj, "path".to_string()) {
        Some(f) => match f {
            RuntimeObject::Str(s) => s,
            _ => return Err(format!("Object Error Path must be string"))
        }
        None => return Err(format!("File Object must contain "))
    };
    let content = get_as_string(args, 1)?;

    match File::open(path) {
        Ok( mut f) => {
            f.write_fmt(format_args!("{}", content)).expect("cannot write io error");
            Ok(RuntimeObject::Void)
        }
        Err(_) => Err(format!("File cannot be opend"))
    }
}


fn d_format(obj: &RuntimeObject, storage: &mut ObjectStorage) -> String {
    match obj {
        /*
        RuntimeObject::Object(o) => {
            let mut str = "{\n".to_string();
            for (_, (k,v)) in storage.borow_fields(o).iter().enumerate() {
                     str+=format!("{}: {}\n", k, d_format(v, storage)).as_str();
            }
            str+"}"
        },
         */
        RuntimeObject::Object(_) => format!("Object"),
        RuntimeObject::Num(n) => n.to_string(),
        RuntimeObject::List(list) => {
            let mut start = "[".to_string();
            list.iter().for_each(|it| {
                start+= &d_format(it, storage);
            });
            start+"]"
        }
        RuntimeObject::Str(s) => s.to_string(),
        RuntimeObject::Bool(b) => b.to_string(),
        RuntimeObject::Void => format!("Any"),
    }
}