use std::io::{self, Write};

use crate::parser::run_expr;
pub struct Repl;

impl Repl {
    pub fn start() {
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
    
            let trimmed = input.trim();
            if trimmed == ":quit" {
                break;
            }
            run_expr(trimmed);
        }
    
    }
}