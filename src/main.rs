use parser::parse_expr;

mod parser;

fn main() {
    let input = "2 + 3 * (4 - 1) * (4 + 2) + sqrt2(1+2) + add(1,2) ";
    match parse_expr(input) {
        Ok((_, expr)) => {
            println!("Parsed expression: {} ", expr);
        }
        Err(err) => println!("Error parsing expression: {:?}", err),
    }
}
