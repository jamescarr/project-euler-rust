// 
// The prime factors of 13195 are 5, 7, 13 and 29.
// 
// What is the largest prime factor of the number 600851475143 ?
pub fn solve() {
    let result = largest_prime_factor(600851475143);

    println!("The largest prime factor of the number 600851475143 is {}", result);
}

pub fn largest_prime_factor(n: u64) -> u64 {
    let mut factors = n;
    let mut max_prime = 0;

    // Print the number of 2s that divide n
    while factors % 2 == 0 {
        max_prime = 2;
        factors /= 2;
    }

    // n must be odd at this point so a skip of 2 can be used
    let mut i = 3;
    while i <= (factors as f64).sqrt() as u64 {
        // while i divides n , print i and divide n
        while factors % i == 0 {
            max_prime = i;
            factors /= i;
        }
        i += 2;
    }

    // This condition is to handle the case when n is a prime number
    // greater than 2
    if factors > 2 {
        max_prime = factors;
    }

    max_prime
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
