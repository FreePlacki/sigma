use crate::error::ErrorKind;
use crate::gen_error;
use crate::tokens::Token;
use crate::{error::Error, value::Value};

trait Function {
    fn get_arity() -> usize;
    fn require_dimensionless() -> bool {
        false
    }
    fn check_domain(_: &Vec<Value>) -> bool {
        true
    }
    fn apply(arguments: Vec<Value>) -> Value;
}

struct Sqrt {}
struct Nthroot {}
struct Sin {}
struct Cos {}
struct Tan {}
struct Asin {}
struct Acos {}
struct Atan {}
struct Ln {}
struct Log {}

impl Function for Sqrt {
    fn get_arity() -> usize {
        1
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let number = arguments[0].number.sqrt();
        let dimension = arguments[0].dimension.as_ref().map(|dim| dim.pow_dim(0.5));
        Value { number, dimension }
    }
}

impl Function for Nthroot {
    fn get_arity() -> usize {
        2
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let power = 1.0 / arguments[1].number;
        let number = arguments[0].number.powf(power);
        let dimension = arguments[0]
            .dimension
            .as_ref()
            .map(|dim| dim.pow_dim(power));
        Value { number, dimension }
    }
}

impl Function for Sin {
    fn get_arity() -> usize {
        1
    }
    fn require_dimensionless() -> bool {
        true
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let number = arguments[0].number.sin();
        Value {
            number,
            dimension: None,
        }
    }
}

impl Function for Cos {
    fn get_arity() -> usize {
        1
    }
    fn require_dimensionless() -> bool {
        true
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let number = arguments[0].number.cos();
        Value {
            number,
            dimension: None,
        }
    }
}

impl Function for Tan {
    fn get_arity() -> usize {
        1
    }
    fn require_dimensionless() -> bool {
        true
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let number = arguments[0].number.tan();
        Value {
            number,
            dimension: None,
        }
    }
}

impl Function for Asin {
    fn get_arity() -> usize {
        1
    }
    fn require_dimensionless() -> bool {
        true
    }
    fn check_domain(arguments: &Vec<Value>) -> bool {
        (-1.0..=1.0).contains(&arguments[0].number)
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let number = arguments[0].number.asin();
        Value {
            number,
            dimension: None,
        }
    }
}

impl Function for Acos {
    fn get_arity() -> usize {
        1
    }
    fn require_dimensionless() -> bool {
        true
    }
    fn check_domain(arguments: &Vec<Value>) -> bool {
        (-1.0..=1.0).contains(&arguments[0].number)
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let number = arguments[0].number.acos();
        Value {
            number,
            dimension: None,
        }
    }
}

impl Function for Atan {
    fn get_arity() -> usize {
        1
    }
    fn require_dimensionless() -> bool {
        true
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let number = arguments[0].number.atan();
        Value {
            number,
            dimension: None,
        }
    }
}

impl Function for Ln {
    fn get_arity() -> usize {
        1
    }
    fn require_dimensionless() -> bool {
        true
    }
    fn check_domain(arguments: &Vec<Value>) -> bool {
        arguments[0].number > 0.0
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let number = arguments[0].number.ln();
        Value {
            number,
            dimension: None,
        }
    }
}

impl Function for Log {
    fn get_arity() -> usize {
        2
    }
    fn require_dimensionless() -> bool {
        true
    }
    fn check_domain(arguments: &Vec<Value>) -> bool {
        arguments[0].number != 1.0 && arguments[0].number > 0.0 && arguments[1].number > 0.0
    }

    fn apply(arguments: Vec<Value>) -> Value {
        let base = arguments[0].number;
        let number = arguments[1].number.log(base);
        Value {
            number,
            dimension: None,
        }
    }
}

fn apply_function<F: Function>(
    _function: F,
    arguments: Vec<Value>,
    name: Token,
) -> Result<Value, Error> {
    if arguments.len() != F::get_arity() {
        return Err(gen_error!(
            ErrorKind::InvalidNumberOfArgs(name.lexeme, F::get_arity(), arguments.len()),
            name
        ));
    }

    if F::require_dimensionless() && arguments.iter().any(|arg| !arg.is_dimensionless()) {
        return Err(gen_error!(
            ErrorKind::ExpectDimensionless(name.lexeme),
            name
        ));
    }

    if !F::check_domain(&arguments) {
        return Err(gen_error!(ErrorKind::InvalidDomain(name.lexeme), name));
    }

    Ok(F::apply(arguments))
}

pub fn eval_function(name: Token, arguments: Vec<Value>) -> Result<Value, Error> {
    match name.lexeme.as_str() {
        "sqrt" => apply_function(Sqrt {}, arguments, name),
        "nthroot" => apply_function(Nthroot {}, arguments, name),
        "sin" => apply_function(Sin {}, arguments, name),
        "cos" => apply_function(Cos {}, arguments, name),
        "tan" => apply_function(Tan {}, arguments, name),
        "asin" => apply_function(Asin {}, arguments, name),
        "acos" => apply_function(Acos {}, arguments, name),
        "atan" => apply_function(Atan {}, arguments, name),
        "ln" => apply_function(Ln {}, arguments, name),
        "log" => apply_function(Log {}, arguments, name),
        _ => Err(gen_error!(ErrorKind::UndefinedFunction, name)),
    }
}
