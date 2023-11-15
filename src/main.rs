use std::f32::consts::PI;

use functions::FunctionTable;
use lazy_static::lazy_static;

use crate::functions::Function;


mod repl;
mod functions;
mod parser;


lazy_static! {
    static ref FUNCTION_TABLE: FunctionTable = {
        let mut ft = FunctionTable::new();
        ft.add_function("cos",
            Function {
                name: "cos".to_string(),
                func: |a| a[0].cos(),
                arg_count: 1,
            },
        );
        ft.add_function("sin",
            Function {
                name: "sin".to_string(),
                func: |a| a[0].sin(),
                arg_count: 1,
            },
        );
        ft.add_function("tan",
            Function {
                name: "tan".to_string(),
                func: |a| a[0].tan(),
                arg_count: 1,
            },
        );
        ft.add_function("acos",
            Function {
                name: "acos".to_string(),
                func: |a| a[0].acos(),
                arg_count: 1,
            },
        );
        ft.add_function("asin",
            Function {
                name: "asin".to_string(),
                func: |a| a[0].asin(),
                arg_count: 1,
            },
        );
        ft.add_function("atan",
            Function {
                name: "atan".to_string(),
                func: |a| a[0].atan(),
                arg_count: 1,
            },
        );
        ft
    };
}


fn main() {
    repl::Repl::start(); 
    // println!("{:#?}", *FUNCTION_TABLE);
}
