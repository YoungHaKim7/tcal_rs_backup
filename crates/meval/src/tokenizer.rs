//! Tokenizer that converts a mathematical expression in a string form into a series of `Token`s.
//!
//! The parser uses a hand-written recursive descent parser instead of parser combinators.
//!
//! The parser should tokenize only well-formed expressions.
use std;
use std::fmt;

/// An error reported by the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// A token that is not allowed at the given location (contains the location of the offending
    /// character in the source string).
    UnexpectedToken(usize),
    /// Missing right parentheses at the end of the source string (contains the number of missing
    /// parens).
    MissingRParen(i32),
    /// Missing operator or function argument at the end of the expression.
    MissingArgument,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::UnexpectedToken(i) => write!(f, "Unexpected token at byte {}.", i),
            ParseError::MissingRParen(i) => write!(
                f,
                "Missing {} right parenthes{}.",
                i,
                if i == 1 { "is" } else { "es" }
            ),
            ParseError::MissingArgument => write!(f, "Missing argument at the end of expression."),
        }
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnexpectedToken(_) => "unexpected token",
            ParseError::MissingRParen(_) => "missing right parenthesis",
            ParseError::MissingArgument => "missing argument",
        }
    }
}

/// Mathematical operations.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Plus,
    Minus,
    Times,
    Div,
    Rem,
    Pow,
    Fact,
}

/// Expression tokens.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Binary operation.
    Binary(Operation),
    /// Unary operation.
    Unary(Operation),

    /// Left parenthesis.
    LParen,
    /// Right parenthesis.
    RParen,
    /// Comma: function argument separator
    Comma,

    /// A number.
    Number(f64),
    /// A variable.
    Var(String),
    /// A function with name and number of arguments.
    Func(String, Option<usize>),
}

/// Check if a byte is a digit (0-9)
fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

/// Check if a byte is a letter (a-z, A-Z)
fn is_alpha(c: u8) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')
}

/// Check if a byte can start an identifier (letter or underscore)
fn is_ident_start(c: u8) -> bool {
    is_alpha(c) || c == b'_'
}

/// Check if a byte can continue an identifier (letter, digit, or underscore)
fn is_ident_continue(c: u8) -> bool {
    is_alpha(c) || is_digit(c) || c == b'_'
}

/// Check if a byte is whitespace
fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r'
}

/// Parse an identifier starting at position i.
/// Returns the identifier and the position after it.
fn parse_ident(input: &[u8], i: usize) -> Option<(&str, usize)> {
    if i >= input.len() || !is_ident_start(input[i]) {
        return None;
    }

    let mut end = i + 1;
    while end < input.len() && is_ident_continue(input[end]) {
        end += 1;
    }

    std::str::from_utf8(&input[i..end]).ok().map(|s| (s, end))
}

/// Parse a number (integer or float) starting at position i.
/// Returns the number as a Token and the position after it.
fn parse_number(input: &[u8], i: usize) -> Option<(Token, usize)> {
    if i >= input.len() || !is_digit(input[i]) {
        return None;
    }

    let mut end = i;

    // Parse integer part
    while end < input.len() && is_digit(input[end]) {
        end += 1;
    }

    // Parse fractional part
    if end < input.len() && input[end] == b'.' {
        end += 1;
        while end < input.len() && is_digit(input[end]) {
            end += 1;
        }
    }

    // Parse exponent part
    if end < input.len() && (input[end] == b'e' || input[end] == b'E') {
        end += 1;
        if end < input.len() && (input[end] == b'+' || input[end] == b'-') {
            end += 1;
        }
        if end < input.len() && is_digit(input[end]) {
            while end < input.len() && is_digit(input[end]) {
                end += 1;
            }
        } else {
            // Invalid exponent format
            return None;
        }
    }

    let s = std::str::from_utf8(&input[i..end]).ok()?;
    s.parse::<f64>().ok().map(|f| (Token::Number(f), end))
}

/// Parse a function call (identifier followed by '(') starting at position i.
/// Returns the function name and the position after the '('.
fn parse_func(input: &[u8], i: usize) -> Option<(Token, usize)> {
    if let Some((name, mut end)) = parse_ident(input, i) {
        // Skip whitespace
        while end < input.len() && is_whitespace(input[end]) {
            end += 1;
        }

        // Check for '('
        if end < input.len() && input[end] == b'(' {
            return Some((Token::Func(name.to_string(), None), end + 1));
        }
    }
    None
}

/// Skip whitespace starting at position i.
/// Returns the position after the whitespace.
fn skip_whitespace(input: &[u8], i: usize) -> usize {
    let mut pos = i;
    while pos < input.len() && is_whitespace(input[pos]) {
        pos += 1;
    }
    pos
}

/// Tokenize a given mathematical expression.
///
/// The parser should return `Ok` only if the expression is well-formed.
///
/// # Failure
///
/// Returns `Err` if the expression is not well-formed.
pub fn tokenize<S: AsRef<str>>(input: S) -> Result<Vec<Token>, ParseError> {
    use self::TokenizerState::*;

    let mut state = LExpr;
    let mut paren_stack = vec![];
    let mut res = vec![];

    let input = input.as_ref().as_bytes();
    let mut pos = 0;

    while pos < input.len() {
        let pos_after_ws = skip_whitespace(input, pos);

        // If we've reached the end of the input after skipping whitespace, we're done
        if pos_after_ws >= input.len() {
            break;
        }

        let token_result = match (state, paren_stack.last()) {
            (LExpr, _) => {
                // Try to parse: number, function, variable, unary +/-, or '('
                if let Some((num, end)) = parse_number(input, pos_after_ws) {
                    Ok((num, end))
                } else if let Some((func, end)) = parse_func(input, pos_after_ws) {
                    Ok((func, end))
                } else if let Some((name, end)) = parse_ident(input, pos_after_ws) {
                    Ok((Token::Var(name.to_string()), end))
                } else if input[pos_after_ws] == b'+' {
                    Ok((Token::Unary(Operation::Plus), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'-' {
                    Ok((Token::Unary(Operation::Minus), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'(' {
                    Ok((Token::LParen, pos_after_ws + 1))
                } else {
                    Err(pos_after_ws)
                }
            }
            (AfterRExpr, None) => {
                // Try to parse: fact, binary op, or ')'
                if input[pos_after_ws] == b'!' {
                    Ok((Token::Unary(Operation::Fact), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'+' {
                    Ok((Token::Binary(Operation::Plus), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'-' {
                    Ok((Token::Binary(Operation::Minus), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'*' {
                    Ok((Token::Binary(Operation::Times), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'/' {
                    Ok((Token::Binary(Operation::Div), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'%' {
                    Ok((Token::Binary(Operation::Rem), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'^' {
                    Ok((Token::Binary(Operation::Pow), pos_after_ws + 1))
                } else if input[pos_after_ws] == b')' {
                    Ok((Token::RParen, pos_after_ws + 1))
                } else {
                    Err(pos_after_ws)
                }
            }
            (AfterRExpr, Some(&ParenState::Subexpr)) => {
                // Try to parse: fact, binary op, ')', or ','
                if input[pos_after_ws] == b'!' {
                    Ok((Token::Unary(Operation::Fact), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'+' {
                    Ok((Token::Binary(Operation::Plus), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'-' {
                    Ok((Token::Binary(Operation::Minus), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'*' {
                    Ok((Token::Binary(Operation::Times), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'/' {
                    Ok((Token::Binary(Operation::Div), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'%' {
                    Ok((Token::Binary(Operation::Rem), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'^' {
                    Ok((Token::Binary(Operation::Pow), pos_after_ws + 1))
                } else if input[pos_after_ws] == b')' {
                    Ok((Token::RParen, pos_after_ws + 1))
                } else if input[pos_after_ws] == b',' {
                    Ok((Token::Comma, pos_after_ws + 1))
                } else {
                    Err(pos_after_ws)
                }
            }
            (AfterRExpr, Some(&ParenState::Func)) => {
                // Try to parse: fact, binary op, ')', or ','
                if input[pos_after_ws] == b'!' {
                    Ok((Token::Unary(Operation::Fact), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'+' {
                    Ok((Token::Binary(Operation::Plus), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'-' {
                    Ok((Token::Binary(Operation::Minus), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'*' {
                    Ok((Token::Binary(Operation::Times), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'/' {
                    Ok((Token::Binary(Operation::Div), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'%' {
                    Ok((Token::Binary(Operation::Rem), pos_after_ws + 1))
                } else if input[pos_after_ws] == b'^' {
                    Ok((Token::Binary(Operation::Pow), pos_after_ws + 1))
                } else if input[pos_after_ws] == b')' {
                    Ok((Token::RParen, pos_after_ws + 1))
                } else if input[pos_after_ws] == b',' {
                    Ok((Token::Comma, pos_after_ws + 1))
                } else {
                    Err(pos_after_ws)
                }
            }
        };

        match token_result {
            Ok((t, np)) => {
                match t {
                    Token::LParen => {
                        paren_stack.push(ParenState::Subexpr);
                    }
                    Token::Func(..) => {
                        paren_stack.push(ParenState::Func);
                    }
                    Token::RParen => {
                        if paren_stack.pop().is_none() {
                            return Err(ParseError::UnexpectedToken(pos_after_ws));
                        }
                    }
                    Token::Var(_) | Token::Number(_) => {
                        state = AfterRExpr;
                    }
                    Token::Binary(_) | Token::Comma => {
                        state = LExpr;
                    }
                    // Unary tokens, LParen, and Func don't change the state
                    // because we still need to parse their operands
                    _ => {}
                }
                res.push(t);
                pos = np;
            }
            Err(error_pos) => {
                return Err(ParseError::UnexpectedToken(error_pos));
            }
        }
    }

    match state {
        LExpr => Err(ParseError::MissingArgument),
        _ if !paren_stack.is_empty() => Err(ParseError::MissingRParen(paren_stack.len() as i32)),
        _ => Ok(res),
    }
}

#[derive(Debug, Clone, Copy)]
enum TokenizerState {
    // accept any token that is an expression from the left: var, num, (, negpos
    LExpr,
    // accept any token that needs an expression on the left: fact, binop, ), comma
    AfterRExpr,
}

#[derive(Debug, Clone, Copy)]
enum ParenState {
    Subexpr,
    Func,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Operation::*;
    use super::Token::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("a"), Ok(vec![Var("a".into())]));

        assert_eq!(
            tokenize("2 +(3--2) "),
            Ok(vec![
                Number(2f64),
                Binary(Plus),
                LParen,
                Number(3f64),
                Binary(Minus),
                Unary(Minus),
                Number(2f64),
                RParen
            ])
        );

        assert_eq!(
            tokenize("-2^ ab0 *12 - C_0"),
            Ok(vec![
                Unary(Minus),
                Number(2f64),
                Binary(Pow),
                Var("ab0".into()),
                Binary(Times),
                Number(12f64),
                Binary(Minus),
                Var("C_0".into()),
            ])
        );

        assert_eq!(
            tokenize("-sin(pi * 3)^ cos(2) / Func2(x, f(y), z) * _buildIN(y)"),
            Ok(vec![
                Unary(Minus),
                Func("sin".into(), None),
                Var("pi".into()),
                Binary(Times),
                Number(3f64),
                RParen,
                Binary(Pow),
                Func("cos".into(), None),
                Number(2f64),
                RParen,
                Binary(Div),
                Func("Func2".into(), None),
                Var("x".into()),
                Comma,
                Func("f".into(), None),
                Var("y".into()),
                RParen,
                Comma,
                Var("z".into()),
                RParen,
                Binary(Times),
                Func("_buildIN".into(), None),
                Var("y".into()),
                RParen,
            ])
        );

        assert_eq!(
            tokenize("2 % 3"),
            Ok(vec![Number(2f64), Binary(Rem), Number(3f64)])
        );

        assert_eq!(
            tokenize("1 + 3! + 1"),
            Ok(vec![
                Number(1f64),
                Binary(Plus),
                Number(3f64),
                Unary(Fact),
                Binary(Plus),
                Number(1f64)
            ])
        );

        assert_eq!(tokenize("!3"), Err(ParseError::UnexpectedToken(0)));

        assert_eq!(tokenize("()"), Err(ParseError::UnexpectedToken(1)));

        assert_eq!(tokenize(""), Err(ParseError::MissingArgument));
        assert_eq!(tokenize("2)"), Err(ParseError::UnexpectedToken(1)));
        assert_eq!(tokenize("2^"), Err(ParseError::MissingArgument));
        assert_eq!(tokenize("(((2)"), Err(ParseError::MissingRParen(2)));
        assert_eq!(tokenize("f(2,)"), Err(ParseError::UnexpectedToken(4)));
        assert_eq!(tokenize("f(,2)"), Err(ParseError::UnexpectedToken(2)));
    }

    #[test]
    fn test_numbers() {
        assert_eq!(parse_number(b"32143", 0), Some((Token::Number(32143.0), 5)));
        assert_eq!(parse_number(b"2.", 0), Some((Token::Number(2.0), 2)));
        assert_eq!(parse_number(b"32143.25", 0), Some((Token::Number(32143.25), 8)));
        assert_eq!(parse_number(b"0.125e9", 0), Some((Token::Number(0.125e9), 7)));
        assert_eq!(parse_number(b"20.5E-3", 0), Some((Token::Number(20.5E-3), 7)));
        assert_eq!(parse_number(b"123423e+50", 0), Some((Token::Number(123423e+50), 10)));
        assert_eq!(parse_number(b"", 0), None);
        assert_eq!(parse_number(b".2", 0), None);
        assert_eq!(parse_number(b"+", 0), None);
        assert_eq!(parse_number(b"e", 0), None);
        assert_eq!(parse_number(b"1E", 0), None);
        assert_eq!(parse_number(b"1e+", 0), None);
    }

    #[test]
    fn test_ident() {
        assert_eq!(parse_ident(b"abc", 0), Some(("abc", 3)));
        assert_eq!(parse_ident(b"U0", 0), Some(("U0", 2)));
        assert_eq!(parse_ident(b"_034", 0), Some(("_034", 4)));
        assert_eq!(parse_ident(b"a_be45EA", 0), Some(("a_be45EA", 8)));
        assert_eq!(parse_ident(b"aAzZ_", 0), Some(("aAzZ_", 5)));
        assert_eq!(parse_ident(b"", 0), None);
        assert_eq!(parse_ident(b"0", 0), None);
    }

    #[test]
    fn test_func() {
        assert_eq!(parse_func(b"abc(", 0), Some((Token::Func("abc".into(), None), 4)));
        assert_eq!(parse_func(b"abc (", 0), Some((Token::Func("abc".into(), None), 5)));
        assert_eq!(parse_func(b"u0(", 0), Some((Token::Func("u0".into(), None), 3)));
        assert_eq!(parse_func(b"_034 (", 0), Some((Token::Func("_034".into(), None), 6)));
        assert_eq!(parse_func(b"A_be45EA  (", 0), Some((Token::Func("A_be45EA".into(), None), 11)));
        assert_eq!(parse_func(b"", 0), None);
        assert_eq!(parse_func(b"(", 0), None);
        assert_eq!(parse_func(b"0(", 0), None);
    }
}
