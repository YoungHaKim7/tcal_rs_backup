//! Implementation of the shunting-yard algorithm for converting an infix expression to an
//! expression in reverse Polish notation (RPN).
//!
//! See the Wikipedia articles on the [shunting-yard algorithm][shunting] and on [reverse Polish
//! notation][RPN] for more details.
//!
//! [RPN]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
//! [shunting]: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
use std;
use std::fmt;
use crate::tokenizer::Token;

#[derive(Debug, Clone, Copy)]
enum Associativity {
    Left,
    Right,
    NA,
}

/// An error produced by the shunting-yard algorightm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RPNError {
    /// An extra left parenthesis was found.
    MismatchedLParen(usize),
    /// An extra right parenthesis was found.
    MismatchedRParen(usize),
    /// Comma that is not separating function arguments.
    UnexpectedComma(usize),
    /// Too few operands for some operator.
    NotEnoughOperands(usize),
    /// Too many operands reported.
    TooManyOperands,
}

impl fmt::Display for RPNError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RPNError::MismatchedLParen(i) => {
                write!(f, "Mismatched left parenthesis at token {}.", i)
            }
            RPNError::MismatchedRParen(i) => {
                write!(f, "Mismatched right parenthesis at token {}.", i)
            }
            RPNError::UnexpectedComma(i) => write!(f, "Unexpected comma at token {}", i),
            RPNError::NotEnoughOperands(i) => write!(f, "Missing operands at token {}", i),
            RPNError::TooManyOperands => {
                write!(f, "Too many operands left at the end of expression.")
            }
        }
    }
}

impl std::error::Error for RPNError {
    fn description(&self) -> &str {
        match *self {
            RPNError::MismatchedLParen(_) => "mismatched left parenthesis",
            RPNError::MismatchedRParen(_) => "mismatched right parenthesis",
            RPNError::UnexpectedComma(_) => "unexpected comma",
            RPNError::NotEnoughOperands(_) => "missing operands",
            RPNError::TooManyOperands => "too many operands left at the end of expression",
        }
    }
}

/// Returns the operator precedence and associativity for a given token.
fn prec_assoc(token: &Token) -> (u32, Associativity) {
    use self::Associativity::*;
    use crate::tokenizer::{Operation, Token};
    match *token {
        Token::Binary(op) => match op {
            Operation::Plus | Operation::Minus => (1, Left),
            Operation::Times | Operation::Div | Operation::Rem => (2, Left),
            Operation::Pow => (4, Right),
            _ => unimplemented!(),
        },
        Token::Unary(op) => match op {
            Operation::Plus | Operation::Minus => (3, NA),
            Operation::Fact => (5, NA),
            _ => unimplemented!(),
        },
        Token::Var(_) | Token::Number(_) | Token::Func(..) | Token::LParen | Token::RParen | Token::Comma => (0, NA),
    }
}

/// Converts a tokenized infix expression to reverse Polish notation.
///
/// # Failure
///
/// Returns `Err` if the input expression is not well-formed.
pub fn to_rpn(input: &[Token]) -> Result<Vec<Token>, RPNError> {
    use crate::tokenizer::Token;

    let mut output = Vec::with_capacity(input.len());
    let mut stack = Vec::with_capacity(input.len());

    for (index, token) in input.iter().enumerate() {
        let token = token.clone();
        match token {
            Token::Number(_) | Token::Var(_) => output.push(token),
            Token::Unary(_) => stack.push((index, token)),
            Token::Binary(_) => {
                let pa1 = prec_assoc(&token);
                while !stack.is_empty() {
                    let pa2 = prec_assoc(&stack.last().unwrap().1);
                    match (pa1, pa2) {
                        ((i, Associativity::Left), (j, _)) if i <= j => {
                            output.push(stack.pop().unwrap().1);
                        }
                        ((i, Associativity::Right), (j, _)) if i < j => {
                            output.push(stack.pop().unwrap().1);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                stack.push((index, token))
            }
            Token::LParen => stack.push((index, token)),
            Token::RParen => {
                let mut found = false;
                while let Some((_, t)) = stack.pop() {
                    match t {
                        Token::LParen => {
                            found = true;
                            break;
                        }
                        Token::Func(name, nargs) => {
                            found = true;
                            output.push(Token::Func(name, Some(nargs.unwrap_or(0) + 1)));
                            break;
                        }
                        _ => output.push(t),
                    }
                }
                if !found {
                    return Err(RPNError::MismatchedRParen(index));
                }
            }
            Token::Comma => {
                let mut found = false;
                while let Some((i, t)) = stack.pop() {
                    match t {
                        Token::LParen => {
                            return Err(RPNError::UnexpectedComma(index));
                        }
                        Token::Func(name, nargs) => {
                            found = true;
                            stack.push((i, Token::Func(name, Some(nargs.unwrap_or(0) + 1))));
                            break;
                        }
                        _ => output.push(t),
                    }
                }
                if !found {
                    return Err(RPNError::UnexpectedComma(index));
                }
            }
            Token::Func(..) => stack.push((index, token)),
        }
    }

    while let Some((index, token)) = stack.pop() {
        match token {
            Token::Unary(_) | Token::Binary(_) => output.push(token),
            Token::LParen | Token::Func(..) => return Err(RPNError::MismatchedLParen(index)),
            _ => panic!("Unexpected token on stack."),
        }
    }

    // verify rpn
    let mut n_operands = 0isize;
    for (index, token) in output.iter().enumerate() {
        match *token {
            Token::Var(_) | Token::Number(_) => n_operands += 1,
            Token::Unary(_) => (),
            Token::Binary(_) => n_operands -= 1,
            Token::Func(_, Some(n_args)) => n_operands -= n_args as isize - 1,
            _ => panic!("Nothing else should be here"),
        }
        if n_operands <= 0 {
            return Err(RPNError::NotEnoughOperands(index));
        }
    }

    if n_operands > 1 {
        return Err(RPNError::TooManyOperands);
    }

    output.shrink_to_fit();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Operation;
    use crate::tokenizer::Token;

    #[test]
    fn test_to_rpn() {
        assert_eq!(to_rpn(&[Token::Number(1.)]), Ok(vec![Token::Number(1.)]));
        assert_eq!(
            to_rpn(&[Token::Number(1.), Token::Binary(Operation::Plus), Token::Number(2.)]),
            Ok(vec![Token::Number(1.), Token::Number(2.), Token::Binary(Operation::Plus)])
        );
        assert_eq!(
            to_rpn(&[Token::Unary(Operation::Minus), Token::Number(1.), Token::Binary(Operation::Pow), Token::Number(2.)]),
            Ok(vec![Token::Number(1.), Token::Number(2.), Token::Binary(Operation::Pow), Token::Unary(Operation::Minus)])
        );
        assert_eq!(
            to_rpn(&[Token::Number(1.), Token::Unary(Operation::Fact), Token::Binary(Operation::Pow), Token::Number(2.)]),
            Ok(vec![Token::Number(1.), Token::Unary(Operation::Fact), Token::Number(2.), Token::Binary(Operation::Pow)])
        );
        assert_eq!(
            to_rpn(&[
                Token::Number(1.),
                Token::Unary(Operation::Fact),
                Token::Binary(Operation::Div),
                Token::LParen,
                Token::Number(2.),
                Token::Binary(Operation::Plus),
                Token::Number(3.),
                Token::RParen,
                Token::Unary(Operation::Fact)
            ]),
            Ok(vec![
                Token::Number(1.),
                Token::Unary(Operation::Fact),
                Token::Number(2.),
                Token::Number(3.),
                Token::Binary(Operation::Plus),
                Token::Unary(Operation::Fact),
                Token::Binary(Operation::Div)
            ])
        );
        assert_eq!(
            to_rpn(&[
                Token::Number(3.),
                Token::Binary(Operation::Minus),
                Token::Number(1.),
                Token::Binary(Operation::Times),
                Token::Number(2.)
            ]),
            Ok(vec![
                Token::Number(3.),
                Token::Number(1.),
                Token::Number(2.),
                Token::Binary(Operation::Times),
                Token::Binary(Operation::Minus)
            ])
        );
        assert_eq!(
            to_rpn(&[
                Token::LParen,
                Token::Number(3.),
                Token::Binary(Operation::Minus),
                Token::Number(1.),
                Token::RParen,
                Token::Binary(Operation::Times),
                Token::Number(2.)
            ]),
            Ok(vec![
                Token::Number(3.),
                Token::Number(1.),
                Token::Binary(Operation::Minus),
                Token::Number(2.),
                Token::Binary(Operation::Times)
            ])
        );
        assert_eq!(
            to_rpn(&[
                Token::Number(1.),
                Token::Binary(Operation::Minus),
                Token::Unary(Operation::Minus),
                Token::Unary(Operation::Minus),
                Token::Number(2.)
            ]),
            Ok(vec![
                Token::Number(1.),
                Token::Number(2.),
                Token::Unary(Operation::Minus),
                Token::Unary(Operation::Minus),
                Token::Binary(Operation::Minus)
            ])
        );
        assert_eq!(
            to_rpn(&[Token::Var("x".into()), Token::Binary(Operation::Plus), Token::Var("y".into())]),
            Ok(vec![Token::Var("x".into()), Token::Var("y".into()), Token::Binary(Operation::Plus)])
        );

        assert_eq!(
            to_rpn(&[
                Token::Func("max".into(), None),
                Token::Func("sin".into(), None),
                Token::Number(1f64),
                Token::RParen,
                Token::Comma,
                Token::Func("cos".into(), None),
                Token::Number(2f64),
                Token::RParen,
                Token::RParen
            ]),
            Ok(vec![
                Token::Number(1f64),
                Token::Func("sin".into(), Some(1)),
                Token::Number(2f64),
                Token::Func("cos".into(), Some(1)),
                Token::Func("max".into(), Some(2))
            ])
        );

        assert_eq!(to_rpn(&[Token::Binary(Operation::Plus)]), Err(RPNError::NotEnoughOperands(0)));
        assert_eq!(
            to_rpn(&[Token::Func("f".into(), None), Token::Binary(Operation::Plus), Token::RParen]),
            Err(RPNError::NotEnoughOperands(0))
        );
        assert_eq!(
            to_rpn(&[Token::Var("x".into()), Token::Number(1.)]),
            Err(RPNError::TooManyOperands)
        );
        assert_eq!(to_rpn(&[Token::LParen]), Err(RPNError::MismatchedLParen(0)));
        assert_eq!(to_rpn(&[Token::RParen]), Err(RPNError::MismatchedRParen(0)));
        assert_eq!(
            to_rpn(&[Token::Func("sin".into(), None)]),
            Err(RPNError::MismatchedLParen(0))
        );
        assert_eq!(to_rpn(&[Token::Comma]), Err(RPNError::UnexpectedComma(0)));
        assert_eq!(
            to_rpn(&[Token::Func("f".into(), None), Token::Comma]),
            Err(RPNError::MismatchedLParen(0))
        );
        assert_eq!(
            to_rpn(&[Token::Func("f".into(), None), Token::LParen, Token::Comma, Token::RParen]),
            Err(RPNError::UnexpectedComma(2))
        );
    }
}
