use crate::error::ErrorKind;
use crate::gen_error;
use crate::tokens::Token;
use crate::{error::Error, value::Value};

trait Function {
    fn get_arity(&self) -> usize;
    fn require_dimensionless(&self) -> bool {
        false
    }
    fn apply(parameters: Vec<Value>) -> Value;
}

struct Sqrt {}
struct Nthroot {}
struct Sin {}

impl Function for Sqrt {
    fn get_arity(&self) -> usize {
        1
    }

    fn apply(parameters: Vec<Value>) -> Value {
        let number = parameters[0].number.sqrt();
        let dimension = parameters[0].dimension.as_ref().map(|dim| dim.pow_dim(0.5));
        Value { number, dimension }
    }
}

impl Function for Nthroot {
    fn get_arity(&self) -> usize {
        2
    }

    fn apply(parameters: Vec<Value>) -> Value {
        let power = 1.0 / parameters[1].number;
        let number = parameters[0].number.powf(power);
        let dimension = parameters[0]
            .dimension
            .as_ref()
            .map(|dim| dim.pow_dim(power));
        Value { number, dimension }
    }
}

impl Function for Sin {
    fn get_arity(&self) -> usize {
        1
    }
    fn require_dimensionless(&self) -> bool {
        true
    }

    fn apply(parameters: Vec<Value>) -> Value {
        let number = parameters[0].number.sin();
        Value {
            number,
            dimension: None,
        }
    }
}

fn apply_function<F: Function>(
    function: F,
    arguments: Vec<Value>,
    name: Token,
) -> Result<Value, Error> {
    if arguments.len() != function.get_arity() {
        return Err(gen_error!(
            ErrorKind::InvalidNumberOfArgs(name.lexeme, function.get_arity(), arguments.len()),
            name
        ));
    }

    if function.require_dimensionless() && arguments.iter().any(|arg| !arg.is_dimensionless()) {
        return Err(gen_error!(
            ErrorKind::ExpectDimensionless(name.lexeme),
            name
        ));
    }

    Ok(F::apply(arguments))
}

pub fn eval_function(name: Token, arguments: Vec<Value>) -> Result<Value, Error> {
    match name.lexeme.as_str() {
        "sqrt" => apply_function(Sqrt {}, arguments, name),
        "nthroot" => apply_function(Nthroot {}, arguments, name),
        "sin" => apply_function(Sin {}, arguments, name),
        _ => Err(gen_error!(ErrorKind::UndefinedFunction, name)),
    }
}
