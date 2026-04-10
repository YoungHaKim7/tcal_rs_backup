//! Prime number operations
//!
//! Provides various prime-related functions including:
//! - Primality testing
//! - Finding next/previous primes
//! - Getting the nth prime
//! - Counting primes (π function)
//! - Listing all primes up to a value
//! - Bernoulli numbers

use rug::{Integer, integer::IsPrime};
use std::collections::HashMap;

/// Cache for prime count calculations using Legendre's formula
type PrimeCountCache = HashMap<(i64, i64), i64>;

/// Checks if a number is prime using probabilistic primality testing.
///
/// Uses the Miller-Rabin primality test with multiple rounds for accuracy.
/// Returns `ProbablyPrime` for numbers that pass all test rounds.
///
/// # Arguments
///
/// * `n` - The number to check (must be positive)
///
/// # Returns
///
/// `true` if probably prime, `false` if composite
///
/// # Examples
///
/// ```
/// use qalculate::is_prime;
/// use rug::Integer;
///
/// assert!(is_prime(&Integer::from(17)));
/// assert!(!is_prime(&Integer::from(18)));
/// assert!(is_prime(&Integer::from(7919))); // 1000th prime
/// ```
pub fn is_prime(n: &Integer) -> bool {
    if n < &2 {
        return false;
    }

    // Check small primes
    if n < &1_000_000 {
        let n_i64 = n.to_i64_wrapping();
        return is_prime_small(n_i64);
    }

    // Use Miller-Rabin for larger numbers
    matches!(n.is_probably_prime(25), IsPrime::Yes)
}

/// Optimized primality test for small integers.
fn is_prime_small(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Finds the smallest prime greater than or equal to n.
///
/// # Arguments
///
/// * `n` - The starting number (non-negative)
///
/// # Returns
///
/// The next prime number as Integer
///
/// # Examples
///
/// ```
/// use qalculate::next_prime;
/// use rug::Integer;
///
/// assert_eq!(next_prime(&Integer::from(10)), Integer::from(11));
/// assert_eq!(next_prime(&Integer::from(11)), Integer::from(11));
/// assert_eq!(next_prime(&Integer::from(14)), Integer::from(17));
/// ```
pub fn next_prime(n: &Integer) -> Integer {
    if n < &2 {
        return Integer::from(2);
    }

    let mut candidate = n.clone();

    // Ensure we start from an odd number
    if candidate.is_even() {
        candidate += 1;
    }

    loop {
        if is_prime(&candidate) {
            return candidate;
        }
        candidate += 2;
    }
}

/// Finds the largest prime less than or equal to n.
///
/// # Arguments
///
/// * `n` - The starting number (must be >= 2)
///
/// # Returns
///
/// The previous prime number as Integer
///
/// # Examples
///
/// ```
/// use qalculate::prev_prime;
/// use rug::Integer;
///
/// assert_eq!(prev_prime(&Integer::from(10)), Integer::from(7));
/// assert_eq!(prev_prime(&Integer::from(7)), Integer::from(7));
/// ```
pub fn prev_prime(n: &Integer) -> Integer {
    if n <= &2 {
        return Integer::from(2);
    }

    let mut candidate = n.clone();

    // Ensure we start from an odd number
    if candidate.is_even() {
        candidate -= 1;
    } else if is_prime(&candidate) {
        return candidate;
    } else {
        candidate -= 2;
    }

    loop {
        if candidate <= 2 {
            return Integer::from(2);
        }
        if is_prime(&candidate) {
            return candidate;
        }
        candidate -= 2;
    }
}

/// Returns the nth prime number (1-indexed).
///
/// # Arguments
///
/// * `n` - The index of the prime to retrieve (1-based, must be positive)
///
/// # Returns
///
/// The nth prime number
///
/// # Panics
///
/// Panics if n is zero or negative
///
/// # Examples
///
/// ```
/// use qalculate::nth_prime;
/// use rug::Integer;
///
/// assert_eq!(nth_prime(1), 2);   // First prime
/// assert_eq!(nth_prime(2), 3);   // Second prime
/// assert_eq!(nth_prime(5), 11);  // Fifth prime
/// ```
pub fn nth_prime(n: u64) -> Integer {
    assert!(n > 0, "n must be positive");

    if n <= 10000 {
        // Use precomputed small primes
        get_small_prime((n - 1) as usize)
    } else {
        // Use prime number theorem approximation
        // p_n ≈ n * (ln(n) + ln(ln(n)))
        let n_float = n as f64;
        let ln_n = n_float.ln();
        let ln_ln_n = ln_n.ln();
        let estimate = (n_float * (ln_n + ln_ln_n)) as u64;

        let mut candidate = Integer::from(estimate);
        if candidate <= 0 {
            candidate = Integer::from(2);
        }

        // Use a more accurate bound
        let upper = if n > 6 {
            (n as f64 * (ln_n + ln_ln_n)) as u64 + 1000
        } else {
            estimate + 10
        };

        // Search forward from estimate using prime counting
        let mut count = prime_count(&candidate);
        while count < n as i64 {
            candidate = next_prime(&candidate);
            count += 1;
            if candidate > upper {
                break;
            }
        }

        candidate
    }
}

/// Get the nth small prime from the lookup table.
fn get_small_prime(n: usize) -> Integer {
    include!("small_primes.inc")[n].into()
}

/// Counts the number of primes less than or equal to x (π function).
///
/// Uses Legendre's formula for efficient computation:
/// π(x) = φ(x, a) + a - 1, where a = π(√x)
///
/// # Arguments
///
/// * `x` - Upper bound (non-negative)
///
/// # Returns
///
/// The number of primes ≤ x
///
/// # Examples
///
/// ```
/// use qalculate::prime_count;
/// use rug::Integer;
///
/// assert_eq!(prime_count(&Integer::from(10)), 4);  // 2, 3, 5, 7
/// assert_eq!(prime_count(&Integer::from(100)), 25);
/// ```
pub fn prime_count(x: &Integer) -> i64 {
    if x < &2 {
        return 0;
    }

    // Check if x fits in i64 (number of bits)
    let x_bits = x.significant_bits();
    let x_i64 = if x_bits <= 63 {
        x.to_i64_wrapping().abs()
    } else {
        // For very large numbers, use approximation
        return prime_count_approx(x);
    };

    if x_i64 <= 104_729 {
        // Use binary search on precomputed primes
        return count_primes_binary(x_i64);
    }

    // Use Legendre's formula for larger numbers
    let mut cache = PrimeCountCache::new();
    legendre_phi(
        x_i64,
        prime_count_small(&Integer::from((x_i64 as f64).sqrt() as i64)),
        &mut cache,
    ) + prime_count_small(&Integer::from((x_i64 as f64).sqrt() as i64))
        - 1
}

/// Approximate prime count using the logarithmic integral.
fn prime_count_approx(x: &Integer) -> i64 {
    if x < &2 {
        return 0;
    }

    // Use x / ln(x) approximation
    let x_f64 = x.to_f64();
    if x_f64.is_finite() && x_f64 > 1.0 {
        (x_f64 / x_f64.ln()) as i64
    } else {
        i64::MAX
    }
}

/// Count primes using binary search on small primes table.
fn count_primes_binary(x: i64) -> i64 {
    let primes = include!("small_primes.inc");

    match primes.binary_search(&x) {
        Ok(i) => (i + 1) as i64,
        Err(i) => i as i64,
    }
}

/// Prime count for small numbers using sieve.
fn prime_count_small(x: &Integer) -> i64 {
    if x < &2 {
        return 0;
    }

    let x_i64 = x.to_i64_wrapping().min(100000);

    // Sieve of Eratosthenes
    let mut sieve = vec![true; (x_i64 + 1) as usize];
    sieve[0] = false;
    if x_i64 >= 1 {
        sieve[1] = false;
    }

    let mut i = 2;
    while i * i <= x_i64 {
        if sieve[i as usize] {
            let mut j = i * i;
            while j <= x_i64 {
                sieve[j as usize] = false;
                j += i;
            }
        }
        i += 1;
    }

    sieve.iter().filter(|&&is_prime| is_prime).count() as i64
}

/// Legendre's phi function: φ(x, a) = number of integers ≤ x with exactly
/// a prime factors, all > the a-th prime.
///
/// This is a key component of Legendre's formula for prime counting.
fn legendre_phi(x: i64, a: i64, cache: &mut PrimeCountCache) -> i64 {
    if a == 1 {
        return (x + 1) / 2;
    }

    let key = (x, a);
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let primes = include!("small_primes.inc");
    let pa = if (a - 1) as usize >= primes.len() {
        // For large a, we need to compute this prime
        nth_prime(a as u64).to_i64_wrapping()
    } else {
        primes[(a - 1) as usize]
    };

    let result = legendre_phi(x, a - 1, cache) - legendre_phi(x / pa, a - 1, cache);
    cache.insert(key, result);
    result
}

/// Returns all prime numbers up to and including n.
///
/// Uses the Sieve of Eratosthenes for efficiency.
///
/// # Arguments
///
/// * `n` - Upper bound (must be non-negative)
///
/// # Returns
///
/// Vector of all primes ≤ n
///
/// # Examples
///
/// ```
/// use qalculate::primes_up_to;
/// use rug::Integer;
///
/// let primes = primes_up_to(&Integer::from(20));
/// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
/// ```
pub fn primes_up_to(n: &Integer) -> Vec<Integer> {
    if n < &2 {
        return Vec::new();
    }

    let n_i64 = if let Some(v) = n.to_u64() {
        if v > 10_000_000 {
            // For very large n, limit to reasonable size
            return Vec::new();
        }
        v as usize
    } else {
        return Vec::new();
    };

    // Sieve of Eratosthenes
    let mut sieve = vec![true; n_i64 + 1];
    sieve[0] = false;
    sieve[1] = false;

    let mut i = 2;
    while i * i <= n_i64 {
        if sieve[i] {
            let mut j = i * i;
            while j <= n_i64 {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    sieve
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| Integer::from(i as u64))
        .collect()
}

/// Computes the nth Bernoulli number B_n.
///
/// Bernoulli numbers are a sequence of rational numbers that appear
/// in many areas of mathematics including the Taylor series expansion
/// of tan(x) and the Faulhaber formula for sums of powers.
///
/// # Arguments
///
/// * `n` - Index of Bernoulli number (non-negative)
///
/// # Returns
///
/// The Bernoulli number as a Rational (numerator, denominator)
///
/// # Examples
///
/// ```
/// use qalculate::bernoulli;
///
/// // B_0 = 1
/// assert_eq!(bernoulli(0), Some((1, 1)));
/// // B_1 = -1/2
/// assert_eq!(bernoulli(1), Some((-1, 2)));
/// // B_2 = 1/6
/// assert_eq!(bernoulli(2), Some((1, 6)));
/// // B_odd > 1 = 0
/// assert_eq!(bernoulli(3), Some((0, 1)));
/// ```
pub fn bernoulli(n: u64) -> Option<(i64, i64)> {
    // Special case: B_1 = -1/2
    if n == 1 {
        return Some((-1, 2));
    }
    // Special case: B_0 = 1
    if n == 0 {
        return Some((1, 1));
    }
    // For odd n > 1, B_n = 0
    if n % 2 == 1 && n > 1 {
        return Some((0, 1));
    }

    // Use Akiyama-Tanigawa algorithm with proper rational arithmetic
    let m = n as usize;
    // Initialize array: a[j] = 1/(j+1) for j = 0 to m
    let mut a: Vec<(i64, i64)> = (0..=m).map(|j| (1, j as i64 + 1)).collect();

    for m_curr in 1..=m {
        for k in (1..=m_curr).rev() {
            // Compute: a[k-1] = k * (a[k-1] - a[k])
            let (num1, den1) = a[k - 1];
            let (num2, den2) = a[k];

            // (num1/den1) - (num2/den2) = (num1*den2 - num2*den1) / (den1*den2)
            let diff_num = num1 * den2 - num2 * den1;
            let diff_den = den1 * den2;

            // k * (diff_num/diff_den) = (k * diff_num) / diff_den
            let k_val = k as i64;
            let new_num = k_val * diff_num;
            let new_den = diff_den;

            // Simplify the fraction
            let g = gcd_int(new_num.abs(), new_den);
            a[k - 1] = (new_num / g, new_den / g);
        }
    }

    Some(a[0])
}

fn gcd_int(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd_int(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(&Integer::from(2)));
        assert!(is_prime(&Integer::from(3)));
        assert!(is_prime(&Integer::from(17)));
        assert!(is_prime(&Integer::from(7919))); // 1000th prime
        assert!(!is_prime(&Integer::from(1)));
        assert!(!is_prime(&Integer::from(4)));
        assert!(!is_prime(&Integer::from(100)));
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(&Integer::from(10)), Integer::from(11));
        assert_eq!(next_prime(&Integer::from(11)), Integer::from(11));
        assert_eq!(next_prime(&Integer::from(14)), Integer::from(17));
        assert_eq!(next_prime(&Integer::from(0)), Integer::from(2));
    }

    #[test]
    fn test_prev_prime() {
        assert_eq!(prev_prime(&Integer::from(10)), Integer::from(7));
        assert_eq!(prev_prime(&Integer::from(7)), Integer::from(7));
        assert_eq!(prev_prime(&Integer::from(2)), Integer::from(2));
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
        assert_eq!(nth_prime(5), 11);
        assert_eq!(nth_prime(10), 29);
    }

    #[test]
    fn test_prime_count() {
        assert_eq!(prime_count(&Integer::from(0)), 0);
        assert_eq!(prime_count(&Integer::from(1)), 0);
        assert_eq!(prime_count(&Integer::from(2)), 1);
        assert_eq!(prime_count(&Integer::from(10)), 4);
        assert_eq!(prime_count(&Integer::from(100)), 25);
    }

    #[test]
    fn test_primes_up_to() {
        let primes = primes_up_to(&Integer::from(20));
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);

        let primes_empty = primes_up_to(&Integer::from(1));
        assert!(primes_empty.is_empty());
    }

    #[test]
    fn test_bernoulli() {
        assert_eq!(bernoulli(0), Some((1, 1)));
        assert_eq!(bernoulli(1), Some((-1, 2)));
        assert_eq!(bernoulli(2), Some((1, 6)));
        assert_eq!(bernoulli(3), Some((0, 1)));
        assert_eq!(bernoulli(4), Some((-1, 30)));
    }
}
