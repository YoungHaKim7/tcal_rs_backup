//! Extension traits for number theory operations
//!
//! Provides trait implementations for easy use of number theory functions.

use rug::Integer;

/// Trait for computing divisors of an integer.
pub trait Divisors {
    /// Returns all positive divisors of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// use qalculate::number_theory::traits::Divisors;
    /// use rug::Integer;
    ///
    /// let n = Integer::from(12);
    /// let divisors = n.divisors();
    /// assert_eq!(divisors, vec![1, 2, 3, 4, 6, 12]);
    /// ```
    fn divisors(&self) -> Vec<Integer>;
}

/// Trait for computing greatest common divisor.
pub trait Gcd<T> {
    /// Computes the greatest common divisor with another number.
    ///
    /// # Examples
    ///
    /// ```
    /// use qalculate::number_theory::traits::Gcd;
    /// use rug::Integer;
    ///
    /// let a = Integer::from(48);
    /// let b = Integer::from(18);
    /// assert_eq!(a.gcd(&b), Integer::from(6));
    /// ```
    fn gcd(&self, other: &T) -> Integer;
}

/// Trait for computing least common multiple.
pub trait Lcm<T> {
    /// Computes the least common multiple with another number.
    ///
    /// # Examples
    ///
    /// ```
    /// use qalculate::number_theory::traits::Lcm;
    /// use rug::Integer;
    ///
    /// let a = Integer::from(21);
    /// let b = Integer::from(6);
    /// assert_eq!(a.lcm(&b), Integer::from(42));
    /// ```
    fn lcm(&self, other: &T) -> Integer;
}

impl Divisors for Integer {
    fn divisors(&self) -> Vec<Integer> {
        if self.is_zero() {
            return vec![];
        }

        let n = self.clone().abs();
        if n == 1 {
            return vec![Integer::from(1)];
        }

        let mut divisors = Vec::new();

        // Efficient divisor enumeration using prime factorization
        let factors = prime_factorization(&n);

        // Generate all divisors from prime factors
        generate_divisors(&factors, 0, Integer::from(1), &mut divisors);

        divisors.sort();
        divisors.dedup();
        divisors
    }
}

impl Gcd<Integer> for Integer {
    fn gcd(&self, other: &Integer) -> Integer {
        if self.is_zero() {
            return other.clone().abs();
        }
        if other.is_zero() {
            return self.clone().abs();
        }
        self.clone().gcd_ref(other).into()
    }
}

impl Lcm<Integer> for Integer {
    fn lcm(&self, other: &Integer) -> Integer {
        if self.is_zero() || other.is_zero() {
            return Integer::from(0);
        }
        self.clone().lcm_ref(other).into()
    }
}

/// Represents a prime factor with its exponent.
#[derive(Debug, Clone)]
struct PrimeFactor {
    prime: Integer,
    exponent: u32,
}

/// Computes the prime factorization of a positive integer.
///
/// Returns a vector of (prime, exponent) pairs sorted by prime.
fn prime_factorization(n: &Integer) -> Vec<PrimeFactor> {
    if *n <= 1 {
        return Vec::new();
    }

    let mut n = n.clone();
    let mut factors = Vec::new();

    // Check divisibility by 2
    let mut exp = 0u32;
    while n.is_even() {
        exp += 1;
        n /= 2;
    }
    if exp > 0 {
        factors.push(PrimeFactor {
            prime: Integer::from(2),
            exponent: exp,
        });
    }

    // Check odd divisors from 3 onwards
    let mut i = Integer::from(3);
    loop {
        let i_sq: Integer = (&i * &i).into();
        if i_sq > n {
            break;
        }
        exp = 0;
        while n.is_divisible(&i) {
            exp += 1;
            n /= &i;
        }
        if exp > 0 {
            factors.push(PrimeFactor {
                prime: i.clone(),
                exponent: exp,
            });
        }
        i += 2;
    }

    // If n is still greater than 1, then it's a prime factor
    if n > 1 {
        factors.push(PrimeFactor {
            prime: n,
            exponent: 1,
        });
    }

    factors
}

/// Recursively generates all divisors from prime factorization.
fn generate_divisors(
    factors: &[PrimeFactor],
    index: usize,
    current: Integer,
    divisors: &mut Vec<Integer>,
) {
    if index == factors.len() {
        divisors.push(current);
        return;
    }

    let factor = &factors[index];
    let mut power = Integer::from(1);

    for _ in 0..=factor.exponent {
        let next = (&current * &power).into();
        generate_divisors(factors, index + 1, next, divisors);
        power *= &factor.prime;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisors() {
        let n = Integer::from(1);
        assert_eq!(n.divisors(), vec![Integer::from(1)]);

        let n = Integer::from(12);
        assert_eq!(
            n.divisors(),
            vec![1, 2, 3, 4, 6, 12]
                .into_iter()
                .map(Integer::from)
                .collect::<Vec<_>>()
        );

        let n = Integer::from(28);
        assert_eq!(
            n.divisors(),
            vec![1, 2, 4, 7, 14, 28]
                .into_iter()
                .map(Integer::from)
                .collect::<Vec<_>>()
        );

        let n = Integer::from(36);
        assert_eq!(
            n.divisors(),
            vec![1, 2, 3, 4, 6, 9, 12, 18, 36]
                .into_iter()
                .map(Integer::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_gcd_trait() {
        let a = Integer::from(48);
        let b = Integer::from(18);
        assert_eq!(a.gcd(&b), Integer::from(6));

        let a = Integer::from(17);
        let b = Integer::from(23);
        assert_eq!(a.gcd(&b), Integer::from(1));

        let a = Integer::from(0);
        let b = Integer::from(5);
        assert_eq!(a.gcd(&b), Integer::from(5));
    }

    #[test]
    fn test_lcm_trait() {
        let a = Integer::from(21);
        let b = Integer::from(6);
        assert_eq!(a.lcm(&b), Integer::from(42));

        let a = Integer::from(5);
        let b = Integer::from(7);
        assert_eq!(a.lcm(&b), Integer::from(35));
    }

    #[test]
    fn test_prime_factorization() {
        fn check_factors(n: u64, expected: &[(u64, u32)]) {
            let n_int = Integer::from(n);
            let factors = prime_factorization(&n_int);
            assert_eq!(factors.len(), expected.len());
            for (i, (prime, exp)) in expected.iter().enumerate() {
                assert_eq!(factors[i].prime, Integer::from(*prime));
                assert_eq!(factors[i].exponent, *exp);
            }
        }

        check_factors(12, &[(2, 2), (3, 1)]);
        check_factors(28, &[(2, 2), (7, 1)]);
        check_factors(360, &[(2, 3), (3, 2), (5, 1)]);
        check_factors(9973, &[(9973, 1)]); // prime
    }
}
