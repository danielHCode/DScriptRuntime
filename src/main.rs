use std::{process::exit, env};

use parsing::parse_file;
use runtime::execute_std;

use crate::runtime::{EqualityCheck, Function, Operation, Type};


mod runtime;
mod parsing;
mod debugger;

struct RuntimeConfig {
    debug_log: bool,
    debugger: bool,
    functions: Vec<Function>
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let config = match parse_command_args(&args) {
        Ok(v) => v,
        Err(s) => handle_error(s)
    };
    interpret(config);
}

fn parse_command_args(args: &Vec<String>) -> Result<RuntimeConfig, String> {
    let mut debug_log = false;
    let mut debugger = false;
    let mut functions = runtime::std_lib::get_std_library();
    for arg in args {
        match arg.as_str() {
            "--debugLog" => debug_log=true,
            "--debug" => debugger = true,
            source => {
                functions.extend(parse_file(source.to_string())?)
            }
        };
    };

    Ok(RuntimeConfig {debug_log, debugger, functions})
}

fn handle_error(e: String) -> ! {
    println!("{}", e);
    exit(1)
}

fn interpret(config: RuntimeConfig) {

    if config.debug_log {
        println!("\n[Read (1) File]\n");
        config.functions.iter().for_each(|f| {
            println!("Function {}", f.signature);
            println!("Args {:#?}", f.args);
            f.instructions.iter().for_each(|it| println!("{}", it))
        });
        println!("\n\n[Start Execution]\n\n");
    }

    if !config.debugger {
        return execute_std(config.functions, "main");
    }
    debug_shell()

}




