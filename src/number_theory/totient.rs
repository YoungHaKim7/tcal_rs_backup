//! Euler's totient function
//!
//! Provides computation of Euler's totient function φ(n), which counts
//! the positive integers up to n that are relatively prime to n.

use rug::Integer;

/// Computes Euler's totient function φ(n).
///
/// The totient function counts the number of positive integers up to n
/// that are coprime with n (i.e., their greatest common divisor is 1).
///
/// # Formula
///
/// For n = p₁^k₁ × p₂^k₂ × ... × p_m^k_m (prime factorization):
/// φ(n) = n × Π(1 - 1/p_i) for all distinct prime factors p_i
///
/// # Arguments
///
/// * `n` - The integer to compute φ for (must be non-zero)
///
/// # Returns
///
/// φ(n) as an Integer
///
/// # Examples
///
/// ```
/// use qalculate::totient;
/// use rug::Integer;
///
/// assert_eq!(totient(&Integer::from(1)), 1);
/// assert_eq!(totient(&Integer::from(7)), 6);   // 7 is prime
/// assert_eq!(totient(&Integer::from(9)), 6);   // 9 = 3², φ(9) = 9(1-1/3) = 6
/// assert_eq!(totient(&Integer::from(10)), 4);  // Numbers coprime to 10: 1,3,7,9
/// ```
pub fn totient(n: &Integer) -> Integer {
    if n.is_zero() {
        return Integer::from(0);
    }
    if n == &1 {
        return Integer::from(1);
    }

    let mut n_abs = n.clone().abs();
    let mut result = n_abs.clone();
    let original = n_abs.clone();

    // Check divisibility by 2
    if n_abs.is_even() {
        while n_abs.is_even() {
            n_abs /= 2;
        }
        let two = Integer::from(2);
        let quot: Integer = (&result / &two).into();
        result -= quot;
    }

    // Check odd divisors from 3 onwards
    let mut i = Integer::from(3);
    loop {
        let i_sq: Integer = (&i * &i).into();
        if i_sq > original {
            break;
        }
        if n_abs.is_divisible(&i) {
            while n_abs.is_divisible(&i) {
                n_abs /= &i;
            }
            let quot: Integer = (&result / &i).into();
            result -= quot;
        }
        i += 2;
    }

    // If n is still greater than 1, then it's a prime factor
    if n_abs > 1 {
        let quot: Integer = (&result / &n_abs).into();
        result -= quot;
    }

    result
}

/// Computes the totient function using prime factorization.
///
/// This alternative implementation explicitly factors n first.
///
/// # Arguments
///
/// * `n` - The integer to compute φ for
///
/// # Returns
///
/// φ(n) as an Integer, or None if n is zero
pub fn totient_from_factors(n: &Integer) -> Option<Integer> {
    if n.is_zero() {
        return Some(Integer::from(0));
    }
    if n == &1 {
        return Some(Integer::from(1));
    }

    let n_abs = n.clone().abs();
    let factors = prime_factors(&n_abs)?;

    // φ(n) = n × Π(1 - 1/p) for distinct primes p
    let mut result = n_abs;
    for prime in factors {
        let prime_minus_1 = &prime - Integer::from(1);
        let temp: Integer = (&result * &prime_minus_1).into();
        result = (&temp / &prime).into();
    }

    Some(result)
}

/// Returns the distinct prime factors of n.
///
/// # Arguments
///
/// * `n` - The integer to factor (must be positive)
///
/// # Returns
///
/// Vector of distinct prime factors, or None for n = 0
fn prime_factors(n: &Integer) -> Option<Vec<Integer>> {
    if n.is_zero() {
        return None;
    }
    if n == &1 {
        return Some(Vec::new());
    }

    let mut n = n.clone();
    let mut factors = Vec::new();

    // Check divisibility by 2
    while n.is_even() {
        if factors.is_empty() || factors.last() != Some(&Integer::from(2)) {
            factors.push(Integer::from(2));
        }
        n /= 2;
    }

    // Check odd divisors from 3 onwards
    let mut i = Integer::from(3);
    loop {
        let i_sq: Integer = (&i * &i).into();
        if i_sq > n {
            break;
        }
        while n.is_divisible(&i) {
            if factors.last() != Some(&i) {
                factors.push(i.clone());
            }
            n /= &i;
        }
        i += 2;
    }

    // If n is still greater than 1, then it's prime
    if n > 1 {
        factors.push(n);
    }

    Some(factors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totient() {
        // φ(1) = 1
        assert_eq!(totient(&Integer::from(1)), 1);

        // φ(p) = p - 1 for prime p
        assert_eq!(totient(&Integer::from(2)), 1);
        assert_eq!(totient(&Integer::from(3)), 2);
        assert_eq!(totient(&Integer::from(5)), 4);
        assert_eq!(totient(&Integer::from(7)), 6);
        assert_eq!(totient(&Integer::from(11)), 10);

        // φ(p^k) = p^k - p^(k-1) = p^k × (1 - 1/p)
        assert_eq!(totient(&Integer::from(4)), 2); // 4 = 2²
        assert_eq!(totient(&Integer::from(8)), 4); // 8 = 2³
        assert_eq!(totient(&Integer::from(9)), 6); // 9 = 3²
        assert_eq!(totient(&Integer::from(27)), 18); // 27 = 3³

        // φ(ab) = φ(a) × φ(b) for coprime a, b
        assert_eq!(totient(&Integer::from(10)), 4); // φ(2) × φ(5) = 1 × 4 = 4
        assert_eq!(totient(&Integer::from(14)), 6); // φ(2) × φ(7) = 1 × 6 = 6
        assert_eq!(totient(&Integer::from(15)), 8); // φ(3) × φ(5) = 2 × 4 = 8

        // φ(30) = 30 × (1-1/2) × (1-1/3) × (1-1/5) = 30 × 1/2 × 2/3 × 4/5 = 8
        assert_eq!(totient(&Integer::from(30)), 8);
    }

    #[test]
    fn test_totient_negative() {
        // φ(-n) = φ(n) for n > 0
        assert_eq!(totient(&Integer::from(-7)), 6);
        assert_eq!(totient(&Integer::from(-10)), 4);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(&Integer::from(1)), Some(vec![]));
        assert_eq!(
            prime_factors(&Integer::from(2)),
            Some(vec![Integer::from(2)])
        );
        assert_eq!(
            prime_factors(&Integer::from(12)),
            Some(vec![Integer::from(2), Integer::from(3)])
        );
        assert_eq!(
            prime_factors(&Integer::from(30)),
            Some(vec![Integer::from(2), Integer::from(3), Integer::from(5)])
        );
        assert_eq!(prime_factors(&Integer::from(0)), None);
    }
}
