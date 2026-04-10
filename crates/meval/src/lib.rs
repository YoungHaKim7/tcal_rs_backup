use std::fmt;

mod expr;
mod extra_math;
pub mod shunting_yard;
pub mod tokenizer;

#[cfg(feature = "serde")]
pub mod de;

pub use expr::*;
pub use shunting_yard::RPNError;
pub use crate::tokenizer::ParseError;

/// An error produced during parsing or evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnknownVariable(String),
    Function(String, FuncEvalError),
    /// An error returned by the parser.
    ParseError(ParseError),
    /// The shunting-yard algorithm returned an error.
    RPNError(RPNError),
    // A catch all for all other errors during evaluation
    EvalError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnknownVariable(ref name) => {
                write!(f, "Evaluation error: unknown variable `{}`.", name)
            }
            Error::Function(ref name, ref e) => {
                write!(f, "Evaluation error: function `{}`: {}", name, e)
            }
            Error::ParseError(ref e) => {
                write!(f, "Parse error: ")?;
                e.fmt(f)
            }
            Error::RPNError(ref e) => {
                write!(f, "RPN error: ")?;
                e.fmt(f)
            }
            Error::EvalError(ref e) => {
                write!(f, "Eval error: ")?;
                e.fmt(f)
            }
        }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::ParseError(err)
    }
}

impl From<RPNError> for Error {
    fn from(err: RPNError) -> Error {
        Error::RPNError(err)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnknownVariable(_) => "unknown variable",
            Error::Function(_, _) => "function evaluation error",
            Error::EvalError(_) => "eval error",
            Error::ParseError(ref e) => e.description(),
            Error::RPNError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::ParseError(ref e) => Some(e),
            Error::RPNError(ref e) => Some(e),
            Error::Function(_, ref e) => Some(e),
            _ => None,
        }
    }
}
