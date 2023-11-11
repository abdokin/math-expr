use core::fmt;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{alphanumeric1, digit1, multispace0},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(digit1, |s: &str| Expr::Number(s.parse().unwrap()))(input)
}
fn parse_fn_call(input: &str) -> IResult<&str, Expr> {
    let (input, (fn_name, exp)) = preceded(
        multispace0,
        tuple((
            alphanumeric1,
            delimited(
                multispace0,
                preceded(tag("("), separated_list0(char(','), parse_expr)),
                terminated(tag(")"), multispace0),
            ),
        )),
    )(input)?;

    Ok((
        input,
        Expr::FunctionCall {
            name: fn_name.to_string(),
            args: exp,
        },
    ))
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    preceded(
        multispace0,
        alt((
            parse_number,
            parse_fn_call,
            delimited(
                multispace0,
                preceded(tag("("), parse_expr),
                terminated(tag(")"), multispace0),
            ),
        )),
    )(input)
}
fn parse_factor_op(input: &str) -> IResult<&str, &str> {
    map(pair(multispace0, alt((tag("*"), tag("/")))), |(_, op)| op)(input)
}
fn parse_op(input: &str) -> IResult<&str, &str> {
    map(pair(multispace0, alt((tag("+"), tag("-")))), |(_, op)| op)(input)
}
fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_factor(input)?;
    let (input, rest) = many0(pair(parse_factor_op, parse_factor))(input)?;

    let mut expr = init;

    for (op, factor) in rest {
        expr = match op {
            "*" => Expr::Multiply(Box::new(expr), Box::new(factor)),
            "/" => Expr::Divide(Box::new(expr), Box::new(factor)),
            _ => unreachable!(),
        };
    }

    Ok((input, expr))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_term(input)?;
    let (input, rest) = many0(pair(parse_op, parse_term))(input)?;

    let mut expr = init;

    for (op, term) in rest {
        expr = match op {
            "+" => Expr::Add(Box::new(expr), Box::new(term)),
            "-" => Expr::Subtract(Box::new(expr), Box::new(term)),
            _ => unreachable!(),
        };
    }

    Ok((input, expr))
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    FunctionCall { name: String, args: Vec<Expr> },
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(num) => write!(f, "{}", num),
            Expr::Add(left, right) => write!(f, "({} + {})", left, right),
            Expr::Subtract(left, right) => write!(f, "({} - {})", left, right),
            Expr::Multiply(left, right) => write!(f, "({} * {})", left, right),
            Expr::Divide(left, right) => write!(f, "({} / {})", left, right),
            Expr::FunctionCall { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub fn evaluate(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(lhs, rhs) => evaluate(lhs) + evaluate(rhs),
        Expr::Subtract(lhs, rhs) => evaluate(lhs) - evaluate(rhs),
        Expr::Multiply(lhs, rhs) => evaluate(lhs) * evaluate(rhs),
        Expr::Divide(lhs, rhs) => evaluate(lhs) / evaluate(rhs),
        Expr::FunctionCall { name, args } => todo!(),
    }
}


pub fn run_expr(input: &str) -> i64 {
    match parse_expr(input) {
        Ok((_, expr)) => evaluate(&expr),
        Err(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(run_expr("2+3"), 5);
        assert_eq!(run_expr("2+3 + (1+1) + 0"), 7);
    }

    #[test]
    fn test_min() {
        assert_eq!(run_expr("2-3"), -1);
        assert_eq!(run_expr("2-3 - 2"), -3);
    }
}