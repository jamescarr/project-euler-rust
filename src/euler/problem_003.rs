extern crate primal;

// The prime factors of 13195 are 5, 7, 13 and 29.
// 
// What is the largest prime factor of the number 600851475143 ?

pub fn solve() {
    let result = largest_prime_factor(600851475143);

    println!("The largest prime factor of the number 600851475143 is {}", result);
}

pub fn largest_prime_factor(n: u64) -> u64 {
    // Create a sieve with an approximation of the needed size
    let sieve = primal::Sieve::new((n as f64).sqrt() as usize + 1);

    // Use the `factors` method of the sieve to get the prime factors of n
    let factors = sieve.factor(n as usize).unwrap();

    // Get the largest factor
    factors.last().unwrap().0 as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_prime_factor() {
        assert_eq!(largest_prime_factor(2), 2);
    }

    #[test]
    fn test_prime_factor_of_three() {
        assert_eq!(largest_prime_factor(3), 3);
    }

    #[test]
    fn test_largest_prime_factor_of_four() {
        assert_eq!(largest_prime_factor(4), 2);
    }

    #[test]
    fn test_largest_prime_factor_of_nine() {
        assert_eq!(largest_prime_factor(9), 3);
    }

    #[test]
    fn test_largest_prime_factor_of_large_number() {
        assert_eq!(largest_prime_factor(13195), 29);
    }

    #[test]
    fn test_largest_prime_factor_of_very_large_number() {
        assert_eq!(largest_prime_factor(600851475143), 6857);
    }
}
