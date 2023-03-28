use crate::runtime::Function;

use self::files::{parse_sections, open_text_file};

mod files;

pub fn parse_file(file: String) -> Result<Vec<Function>, String> {
    let code = open_text_file(file)?;
    let words = code.replace("\r\n", "\n").replace("\n", " ").split(" ").map(|it| it.to_owned()).collect::<Vec<String>>();
    match parse_sections(&words) {
        Ok(functions) => Ok(functions),
        Err((message, i)) => Err(format!("{}, occured in {} ({})", message, i, words[i]))
    }
}