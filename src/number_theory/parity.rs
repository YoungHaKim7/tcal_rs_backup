//! Parity checking functions
//!
//! Provides functions to check if a number is odd or even.

use rug::Integer;

/// Checks if an integer is even.
///
/// # Arguments
///
/// * `n` - The integer to check
///
/// # Returns
///
/// `true` if the number is even, `false` otherwise
///
/// # Examples
///
/// ```
/// use qalculate::is_even;
///
/// assert!(is_even(4));
/// assert!(!is_even(5));
/// assert!(is_even(0));
/// assert!(is_even(-2));
/// ```
pub fn is_even<T: Into<Integer>>(n: T) -> bool {
    let n: Integer = n.into();
    n.is_divisible_u(2)
}

/// Checks if an integer is odd.
///
/// # Arguments
///
/// * `n` - The integer to check
///
/// # Returns
///
/// `true` if the number is odd, `false` otherwise
///
/// # Examples
///
/// ```
/// use qalculate::is_odd;
///
/// assert!(is_odd(5));
/// assert!(!is_odd(4));
/// assert!(!is_odd(0));
/// assert!(is_odd(-3));
/// ```
pub fn is_odd<T: Into<Integer>>(n: T) -> bool {
    !is_even(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(0i64));
        assert!(is_even(2i64));
        assert!(is_even(-4i64));
        assert!(is_even(100i64));
        assert!(!is_even(1i64));
        assert!(!is_even(-1i64));
        assert!(!is_even(99i64));
    }

    #[test]
    fn test_is_odd() {
        assert!(is_odd(1i64));
        assert!(is_odd(-1i64));
        assert!(is_odd(99i64));
        assert!(!is_odd(0i64));
        assert!(!is_odd(2i64));
        assert!(!is_odd(-4i64));
    }

    #[test]
    fn test_parity_with_integer() {
        let n = Integer::from(42);
        assert!(is_even(&n));
        assert!(!is_odd(&n));
    }
}
