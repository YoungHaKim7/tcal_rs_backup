//! Qalculate - Number Theory Library (Rust)
//!
//! This library provides comprehensive number theory functions, ported from
//! the C++ libqalculate library. It offers efficient implementations for:
//!
//! - Parity checking ([`is_even`], [`is_odd`])
//! - Absolute value ([`abs`], [`abs_integer`])
//! - Rounding operations ([`round`], [`floor`], [`ceil`], [`trunc`])
//! - GCD and LCM ([`gcd`], [`lcm`])
//! - Modular arithmetic ([`modulo`], [`rem`], [`powmod`])
//! - Prime operations ([`is_prime`], [`next_prime`], [`prev_prime`], [`nth_prime`], [`prime_count`])
//! - Euler's totient function ([`totient`])
//! - Bernoulli numbers ([`bernoulli`])
//!
//! # Example
//!
//! ```
//! use qalculate::{gcd, lcm, is_prime, totient};
//! use rug::Integer;
//!
//! let a = Integer::from(48);
//! let b = Integer::from(18);
//! assert_eq!(gcd(&[&a, &b]), Integer::from(6));
//! assert_eq!(lcm(&[&Integer::from(21), &Integer::from(6)]), Integer::from(42));
//! assert!(is_prime(&Integer::from(17)));
//! assert_eq!(totient(&Integer::from(10)), 4);
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod fprice;
pub mod number_theory;

// Re-export common number theory functions for convenience
pub use number_theory::{
    abs, abs_integer, bernoulli, ceil, denominator, floor, frac, gcd, is_even, is_odd, is_prime,
    lcm, modulo, next_prime, nth_prime, numerator, powmod, prev_prime, prime_count, primes_up_to,
    rem, round, rounding::RoundingMode, signum, signum_integer, totient, trunc,
};

pub use fprice::{fprice, fprice_float};
pub use number_theory::traits::{Divisors, Gcd, Lcm};
pub use rug;

/// Result type for fallible operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in number theory operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Division by zero attempted
    DivisionByZero,
    /// Overflow occurred during computation
    Overflow,
    /// Invalid argument provided
    InvalidArgument,
    /// Computation was interrupted (e.g., by user request)
    Interrupted,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::Overflow => write!(f, "arithmetic overflow"),
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::Interrupted => write!(f, "computation interrupted"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::Integer;

    #[test]
    fn test_basic_gcd() {
        let a = Integer::from(48);
        let b = Integer::from(18);
        assert_eq!(gcd(&[&a, &b]), 6);
    }

    #[test]
    fn test_basic_lcm() {
        let a = Integer::from(21);
        let b = Integer::from(6);
        assert_eq!(lcm(&[&a, &b]), 42);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(&Integer::from(17)));
        assert!(!is_prime(&Integer::from(18)));
    }

    #[test]
    fn test_totient() {
        assert_eq!(totient(&Integer::from(10)), 4);
        assert_eq!(totient(&Integer::from(7)), 6);
    }

    #[test]
    fn test_rounding() {
        assert_eq!(round(7, 3, RoundingMode::HalfToEven), 2);
        assert_eq!(round(5, 2, RoundingMode::HalfToEven), 2);
    }
}
