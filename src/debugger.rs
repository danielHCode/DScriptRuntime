use std::{io::stdin, collections::HashMap};
use crate::{Function, runtime::{RuntimeObject, execute_std, Runtime}, RuntimeConfig};

pub fn debug_shell(functions: Vec<Function>) {
    let stdin = stdin();
    let debugger = Debugger {functions, runtime: Runtime::new(), variables: HashMap::new() };
    loop {
        let mut raw = String::new();
        stdin.read_line(&mut raw);
        let words: Vec<&str> = raw.replace("(", " ( ").replace(")", " ) ").split(" ").collect();

        let mut i: usize = 0;
        
    }
}

struct Debugger {
    functions: Vec<Function>,
    variables: HashMap<String, RuntimeObject>,
    runtime: Runtime
}

impl Debugger {
    fn parse_args(&mut self, i: usize, words: &Vec<&str>) -> (usize, RuntimeObject) {
        match words[i] {
            "store" => {
                i+=1;
                let name = words[i];
                (i,_) = self.parse_args(i, words);
                (i, RuntimeObject::Void)
            }
            "exec" => {
                i+=1;
                let sig = words[i];
                match self.runtime.execute(&self.functions, sig, vec![]) {
                    Ok(v) => (i, v),
                    Err(e) => {
                        println!("Error: {}", e);
                        (i, RuntimeObject::Void)
                    }
                }
            }
        }
    }
}

