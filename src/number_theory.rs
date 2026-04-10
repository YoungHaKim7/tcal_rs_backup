//! Number theory functions module
//!
//! Provides various number theory operations including:
//! - Parity checks (odd/even)
//! - Absolute value
//! - Greatest common divisor and least common multiple
//! - Divisors and factorization
//! - Prime number operations
//! - Rounding functions
//! - Modular arithmetic
//! - Euler's totient function
//! - Bernoulli numbers

use rug::Integer;

// Re-exports
pub use self::parity::{is_even, is_odd};
pub use self::primes::{
    bernoulli, is_prime, next_prime, nth_prime, prev_prime, prime_count, primes_up_to,
};
pub use self::rounding::{
    RoundingMode, abs, abs_integer, ceil, floor, round, signum, signum_integer, trunc,
};
pub use self::totient::totient;
pub use self::traits::{Divisors, Gcd, Lcm};

pub mod parity;
pub mod primes;
pub mod rounding;
pub mod totient;
pub mod traits;

/// Computes the greatest common divisor of two or more integers.
///
/// # Arguments
///
/// * `values` - Slice of integers to compute GCD for
///
/// # Returns
///
/// The GCD as an Integer
///
/// # Examples
///
/// ```
/// use tcalulator_rs::gcd;
/// use rug::Integer;
///
/// let a = Integer::from(48);
/// let b = Integer::from(18);
/// let result = gcd(&[&a, &b]);
/// assert_eq!(result, 6);
/// ```
pub fn gcd(values: &[&Integer]) -> Integer {
    if values.is_empty() {
        return Integer::from(0);
    }
    if values.len() == 1 {
        return values[0].clone();
    }

    let mut result = values[0].clone();
    for value in &values[1..] {
        result = result.gcd_ref(value).into();
    }
    result
}

/// Computes the least common multiple of two or more integers.
///
/// # Arguments
///
/// * `values` - Slice of integers to compute LCM for
///
/// # Returns
///
/// The LCM as an Integer
///
/// # Examples
///
/// ```
/// use tcalulator_rs::lcm;
/// use rug::Integer;
///
/// let a = Integer::from(21);
/// let b = Integer::from(6);
/// let result = lcm(&[&a, &b]);
/// assert_eq!(result, 42);
/// ```
pub fn lcm(values: &[&Integer]) -> Integer {
    if values.is_empty() {
        return Integer::from(0);
    }
    if values.len() == 1 {
        return values[0].clone();
    }

    let mut result = values[0].clone();
    for value in &values[1..] {
        result = result.lcm_ref(value).into();
    }
    result
}

/// Computes the fractional part of a number.
///
/// Returns the fractional part of a rational number, i.e., the number
/// minus its integer part.
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
///
/// # Examples
///
/// ```
/// use tcalulator_rs::frac;
///
/// let (num, den) = frac(7, 4);
/// assert_eq!(num, 3);
/// assert_eq!(den, 4);
/// ```
pub fn frac(numerator: i64, denominator: i64) -> (i64, i64) {
    let int_part = numerator / denominator;
    let frac_num = numerator - int_part * denominator;
    (frac_num, denominator)
}

/// Computes the remainder of Euclidean division (rem).
///
/// This differs from `mod` in how it handles negative numbers.
/// The remainder has the same sign as the dividend.
///
/// # Arguments
///
/// * `a` - Dividend
/// * `b` - Divisor (must be non-zero)
///
/// # Returns
///
/// The remainder of a divided by b
///
/// # Examples
///
/// ```
/// use tcalulator_rs::rem;
///
/// assert_eq!(rem(7, 3), 1);
/// assert_eq!(rem(-7, 3), -1);  // Different from mod!
/// ```
pub fn rem(a: i64, b: i64) -> i64 {
    a % b
}

/// Computes the modulo operation.
///
/// The modulo operation always returns a non-negative result,
/// unlike the remainder operation.
///
/// # Arguments
///
/// * `a` - Dividend
/// * `b` - Divisor (must be positive)
///
/// # Returns
///
/// The modulo of a and b
///
/// # Examples
///
/// ```
/// use tcalulator_rs::modulo;
///
/// assert_eq!(modulo(7, 3), 1);
/// assert_eq!(modulo(-7, 3), 2);  // Always non-negative
/// ```
pub fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

/// Computes modular exponentiation (base^exp mod modulus).
///
/// Uses efficient binary exponentiation algorithm.
///
/// # Arguments
///
/// * `base` - The base
/// * `exp` - The exponent (must be non-negative)
/// * `modulus` - The modulus (must be positive)
///
/// # Returns
///
/// (base^exp) mod modulus
///
/// # Examples
///
/// ```
/// use tcalulator_rs::powmod;
/// use rug::Integer;
///
/// let result = powmod(
///     &Integer::from(4),
///     &Integer::from(13),
///     &Integer::from(497)
/// );
/// assert_eq!(result, Integer::from(445));
/// ```
pub fn powmod(base: &Integer, exp: &Integer, modulus: &Integer) -> Integer {
    if *exp < 0 {
        // For negative exponents, we need gcd(base, modulus) == 1
        if base.gcd(modulus) != 1 {
            panic!("powmod with negative exponent requires gcd(base, modulus) == 1");
        }
        // Compute modular inverse first
        let base_inv = base
            .clone()
            .invert(modulus)
            .expect("Modular inverse exists");
        let abs_exp = exp.clone().abs();
        return base_inv.pow_mod(&abs_exp, modulus).expect("Valid pow_mod");
    }

    base.pow_mod_ref(exp, modulus)
        .expect("pow_mod failed")
        .into()
}

/// Returns the numerator of a rational number.
///
/// # Examples
///
/// ```
/// use tcalulator_rs::numerator;
///
/// assert_eq!(numerator(7, 3), 7);
/// assert_eq!(numerator(-7, 3), -7);
/// ```
pub const fn numerator(numerator: i64, _denominator: i64) -> i64 {
    numerator
}

/// Returns the denominator of a rational number.
///
/// # Examples
///
/// ```
/// use tcalulator_rs::denominator;
///
/// assert_eq!(denominator(7, 3), 3);
/// assert_eq!(denominator(-7, 3), 3);
/// ```
pub const fn denominator(_numerator: i64, denominator: i64) -> i64 {
    denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let a = Integer::from(48);
        let b = Integer::from(18);
        assert_eq!(gcd(&[&a, &b]), 6);
        let c = Integer::from(17);
        let d = Integer::from(23);
        assert_eq!(gcd(&[&c, &d]), 1);
        let e = Integer::from(0);
        let f = Integer::from(5);
        assert_eq!(gcd(&[&e, &f]), 5);
        let g = Integer::from(48);
        let h = Integer::from(18);
        let i = Integer::from(30);
        assert_eq!(gcd(&[&g, &h, &i]), 6);
    }

    #[test]
    fn test_lcm() {
        let a = Integer::from(21);
        let b = Integer::from(6);
        assert_eq!(lcm(&[&a, &b]), 42);
        let c = Integer::from(5);
        let d = Integer::from(7);
        assert_eq!(lcm(&[&c, &d]), 35);
        let e = Integer::from(4);
        let f = Integer::from(6);
        let g = Integer::from(8);
        assert_eq!(lcm(&[&e, &f, &g]), 24);
    }

    #[test]
    fn test_frac() {
        assert_eq!(frac(7, 4), (3, 4));
        assert_eq!(frac(-7, 4), (-3, 4));
        assert_eq!(frac(8, 4), (0, 4));
    }

    #[test]
    fn test_rem() {
        assert_eq!(rem(7, 3), 1);
        assert_eq!(rem(-7, 3), -1);
        assert_eq!(rem(7, -3), 1);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(modulo(7, 3), 1);
        assert_eq!(modulo(-7, 3), 2);
        assert_eq!(modulo(7, 5), 2);
    }

    #[test]
    fn test_powmod() {
        assert_eq!(
            powmod(&Integer::from(4), &Integer::from(13), &Integer::from(497)),
            445
        );
        assert_eq!(
            powmod(&Integer::from(2), &Integer::from(10), &Integer::from(1000)),
            24
        );
    }

    #[test]
    fn test_numerator_denominator() {
        assert_eq!(numerator(7, 3), 7);
        assert_eq!(denominator(7, 3), 3);
        assert_eq!(numerator(-7, 3), -7);
        assert_eq!(denominator(-7, 3), 3);
    }
}
