//! Rounding and absolute value functions
//!
//! Provides various rounding operations including:
//! - Absolute value
//! - Ceiling (round up)
//! - Floor (round down)
//! - Truncation (round toward zero)
//! - Rounding to nearest with various modes

use rug::{Float, Integer};

/// Rounding mode for the `round` function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    /// Round half to even (banker's rounding)
    HalfToEven = 0,
    /// Round half away from zero
    HalfAwayFromZero = 1,
    /// Round half toward zero
    HalfTowardZero = 2,
    /// Round half up (toward positive infinity)
    HalfUp = 3,
    /// Round half down (toward negative infinity)
    HalfDown = 4,
    /// Round toward positive infinity (ceiling)
    Up = 5,
    /// Round toward negative infinity (floor)
    Down = 6,
    /// Round toward zero (truncation)
    TowardZero = 7,
    /// Round away from zero
    AwayFromZero = 8,
}

impl Default for RoundingMode {
    fn default() -> Self {
        Self::HalfToEven
    }
}

/// Computes the absolute value of a rational number.
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
///
/// # Returns
///
/// Absolute value as (numerator, denominator)
///
/// # Examples
///
/// ```
/// use qalculate::abs;
///
/// assert_eq!(abs(7, 3), (7, 3));
/// assert_eq!(abs(-7, 3), (7, 3));
/// ```
pub fn abs(numerator: i64, denominator: i64) -> (i64, i64) {
    (numerator.abs(), denominator.abs())
}

/// Computes the absolute value of an Integer.
///
/// # Examples
///
/// ```
/// use qalculate::abs_integer;
/// use rug::Integer;
///
/// assert_eq!(abs_integer(&Integer::from(-42)), Integer::from(42));
/// ```
pub fn abs_integer(n: &Integer) -> Integer {
    n.clone().abs()
}

/// Computes the ceiling of a rational number (round toward +∞).
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
///
/// # Returns
///
/// The ceiling as an i64
///
/// # Examples
///
/// ```
/// use qalculate::ceil;
///
/// assert_eq!(ceil(7, 3), 3);
/// assert_eq!(ceil(-7, 3), -2);
/// assert_eq!(ceil(6, 3), 2);
/// ```
pub fn ceil(numerator: i64, denominator: i64) -> i64 {
    let mut result = numerator / denominator;
    let remainder = numerator % denominator;

    if remainder > 0 && denominator > 0 || remainder < 0 && denominator < 0 {
        result += 1;
    } else if remainder < 0 && denominator > 0 || remainder > 0 && denominator < 0 {
        // Already correct for negative numbers
    }

    result
}

/// Computes the ceiling of a Float.
pub fn ceil_float(f: &Float) -> Integer {
    let s = f.to_string_radix(10, None);
    let float_val: f64 = s.parse().unwrap_or(0.0);
    let mut result = Integer::from(float_val as i64);
    if float_val > 0.0 && float_val.fract() != 0.0 {
        result += 1;
    }
    result
}

/// Computes the floor of a rational number (round toward -∞).
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
///
/// # Returns
///
/// The floor as an i64
///
/// # Examples
///
/// ```
/// use qalculate::floor;
///
/// assert_eq!(floor(7, 3), 2);
/// assert_eq!(floor(-7, 3), -3);
/// assert_eq!(floor(6, 3), 2);
/// ```
pub fn floor(numerator: i64, denominator: i64) -> i64 {
    let mut result = numerator / denominator;
    let remainder = numerator % denominator;

    if remainder < 0 && denominator > 0 || remainder > 0 && denominator < 0 {
        result -= 1;
    }

    result
}

/// Computes the floor of a Float.
pub fn floor_float(f: &Float) -> Integer {
    let s = f.to_string_radix(10, None);
    let float_val: f64 = s.parse().unwrap_or(0.0);
    let mut result = Integer::from(float_val as i64);
    if float_val < 0.0 && float_val.fract() != 0.0 {
        result -= 1;
    }
    result
}

/// Computes the truncation of a rational number (round toward zero).
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
///
/// # Returns
///
/// The truncated value as an i64
///
/// # Examples
///
/// ```
/// use qalculate::trunc;
///
/// assert_eq!(trunc(7, 3), 2);
/// assert_eq!(trunc(-7, 3), -2);
/// assert_eq!(trunc(6, 3), 2);
/// ```
pub fn trunc(numerator: i64, denominator: i64) -> i64 {
    numerator / denominator
}

/// Computes the truncation of a Float.
pub fn trunc_float(f: &Float) -> Integer {
    let s = f.to_string_radix(10, None);
    let float_val: f64 = s.parse().unwrap_or(0.0);
    Integer::from(float_val as i64)
}

/// Rounds a rational number to the nearest integer.
///
/// Uses the specified rounding mode (defaults to HalfToEven).
///
/// # Arguments
///
/// * `numerator` - Numerator of the rational number
/// * `denominator` - Denominator of the rational number
/// * `mode` - Rounding mode to use
///
/// # Returns
///
/// The rounded value as an i64
///
/// # Examples
///
/// ```
/// use qalculate::{round, RoundingMode};
///
/// assert_eq!(round(7, 3, RoundingMode::HalfToEven), 2);  // 2.33... → 2
/// assert_eq!(round(8, 3, RoundingMode::HalfToEven), 3);  // 2.66... → 3
/// assert_eq!(round(5, 2, RoundingMode::HalfToEven), 2);  // 2.5 → 2 (even)
/// assert_eq!(round(7, 2, RoundingMode::HalfToEven), 4);  // 3.5 → 4 (even)
/// ```
pub fn round(numerator: i64, denominator: i64, mode: RoundingMode) -> i64 {
    let integer_part = numerator / denominator;
    let remainder = numerator.abs() % denominator.abs();

    if remainder == 0 {
        return integer_part;
    }

    // Convert to float for precise comparison
    let fraction = (remainder as f64) / (denominator.abs() as f64);

    match mode {
        RoundingMode::HalfToEven => {
            if fraction > 0.5 {
                integer_part + integer_part.signum()
            } else if fraction < 0.5 {
                integer_part
            } else {
                // Exactly at 0.5, round to nearest even
                if integer_part % 2 == 0 {
                    integer_part
                } else {
                    integer_part + integer_part.signum()
                }
            }
        }
        RoundingMode::HalfAwayFromZero => {
            if fraction >= 0.5 {
                integer_part + integer_part.signum()
            } else {
                integer_part
            }
        }
        RoundingMode::HalfTowardZero => {
            if fraction > 0.5 {
                integer_part + integer_part.signum()
            } else {
                integer_part
            }
        }
        RoundingMode::HalfUp => {
            if fraction >= 0.5 {
                integer_part + 1
            } else {
                integer_part
            }
        }
        RoundingMode::HalfDown => {
            if fraction > 0.5 {
                integer_part + 1
            } else {
                integer_part
            }
        }
        RoundingMode::Up => ceil(numerator, denominator),
        RoundingMode::Down => floor(numerator, denominator),
        RoundingMode::TowardZero => trunc(numerator, denominator),
        RoundingMode::AwayFromZero => {
            if integer_part >= 0 {
                ceil(numerator, denominator)
            } else {
                floor(numerator, denominator)
            }
        }
    }
}

/// Signum function - returns the sign of a number.
///
/// Returns:
/// - `-1` if the number is negative
/// - `0` if the number is zero
/// - `1` if the number is positive
///
/// # Examples
///
/// ```
/// use qalculate::signum;
///
/// assert_eq!(signum(-5), -1);
/// assert_eq!(signum(0), 0);
/// assert_eq!(signum(5), 1);
/// ```
pub fn signum(n: i64) -> i64 {
    n.signum()
}

/// Signum function for Integer.
pub fn signum_integer(n: &Integer) -> Integer {
    if n.is_zero() {
        Integer::from(0)
    } else if n.is_positive() {
        Integer::from(1)
    } else {
        Integer::from(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(7, 3), (7, 3));
        assert_eq!(abs(-7, 3), (7, 3));
        assert_eq!(abs(7, -3), (7, 3));
    }

    #[test]
    fn test_ceil() {
        assert_eq!(ceil(7, 3), 3);
        assert_eq!(ceil(-7, 3), -2);
        assert_eq!(ceil(6, 3), 2);
        assert_eq!(ceil(-6, 3), -2);
        assert_eq!(ceil(1, 2), 1);
        assert_eq!(ceil(-1, 2), 0);
    }

    #[test]
    fn test_floor() {
        assert_eq!(floor(7, 3), 2);
        assert_eq!(floor(-7, 3), -3);
        assert_eq!(floor(6, 3), 2);
        assert_eq!(floor(-6, 3), -2);
        assert_eq!(floor(1, 2), 0);
        assert_eq!(floor(-1, 2), -1);
    }

    #[test]
    fn test_trunc() {
        assert_eq!(trunc(7, 3), 2);
        assert_eq!(trunc(-7, 3), -2);
        assert_eq!(trunc(6, 3), 2);
        assert_eq!(trunc(1, 2), 0);
        assert_eq!(trunc(-1, 2), 0);
    }

    #[test]
    fn test_round_half_to_even() {
        assert_eq!(round(7, 3, RoundingMode::HalfToEven), 2);
        assert_eq!(round(8, 3, RoundingMode::HalfToEven), 3);
        assert_eq!(round(5, 2, RoundingMode::HalfToEven), 2);
        assert_eq!(round(7, 2, RoundingMode::HalfToEven), 4);
        assert_eq!(round(9, 2, RoundingMode::HalfToEven), 4);
        assert_eq!(round(11, 2, RoundingMode::HalfToEven), 6);
    }

    #[test]
    fn test_round_half_away_from_zero() {
        assert_eq!(round(7, 3, RoundingMode::HalfAwayFromZero), 2);
        assert_eq!(round(8, 3, RoundingMode::HalfAwayFromZero), 3);
        assert_eq!(round(5, 2, RoundingMode::HalfAwayFromZero), 3);
        assert_eq!(round(-5, 2, RoundingMode::HalfAwayFromZero), -3);
    }

    #[test]
    fn test_signum() {
        assert_eq!(signum(-5), -1);
        assert_eq!(signum(0), 0);
        assert_eq!(signum(5), 1);
    }
}
