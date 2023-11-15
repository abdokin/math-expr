use std::io::{self, Write};

use crate::{parser::run_expr, functions::FunctionTable, FUNCTION_TABLE};
pub struct Repl;

impl Repl {
    pub fn start() {
        let mut function_table = FunctionTable::new();
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
    
            let trimmed = input.trim();
            if trimmed == ":quit" {
                break;
            }else if trimmed == "print fn" {
                println!("All Functions");FUNCTION_TABLE.print_all()
            }else {
                run_expr(trimmed);

            }
        }
    
    }
}

