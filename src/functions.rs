use std::collections::HashMap;
use std::fmt;

pub type MathFunction = fn(Vec<f64>) -> f64;

#[derive(Debug)]
pub struct Function {
    pub func: MathFunction,
    pub name:String,
    pub arg_count: i8,
}

impl Function {
    pub fn evaluate(&self, args: Vec<f64>) -> f64 {
        (self.func)(args)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn: {}",  self.name)
    }
}

#[derive(Debug)]
pub struct FunctionTable {
    functions: HashMap<String, Function>,
}

impl FunctionTable {
    pub fn new() -> Self {
        FunctionTable {
            functions: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, name: &str, func: Function) {
        self.functions.insert(name.to_string(), func);
    }

    pub fn evaluate_function(&self, name: &str, args: Vec<f64>) -> Result<f64, String> {
        if let Some(func) = self.functions.get(name) {
            if args.len() == func.arg_count as usize {
                Ok(func.evaluate(args))
            } else {
                Err(format!("Incorrect number of arguments for function '{}'", name))
            }
        } else {
            Err(format!("Function '{}' not found", name))
        }
    }
    pub fn print_all(&self) {
        for (_,func) in &self.functions {
            println!("{}",func);
        }
    }

}

impl fmt::Display for FunctionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FunctionTable: {:?}", self.functions)
    }
}
