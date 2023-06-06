/// Project Euler Problem 004
/// 
/// A palindromic number reads the same both ways.
/// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
/// Find the largest palindrome made from the product of two 3-digit numbers.

pub fn solve() {
    let largest_palindrome = largest_palindrome_product(100, 999);

    println!("The largest palindrome product of two 3-digit numbers is {}", largest_palindrome);
}

pub fn largest_palindrome_product(min: u32, max: u32) -> u32 {
    let mut max_product = 0;
    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if product > max_product && is_palindrome(product) {
                max_product = product;
            }
        }
    }
    max_product
}

fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let rev_s = s.chars().rev().collect::<String>();
    s == rev_s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_digit_numbers() {
        assert_eq!(largest_palindrome_product(10, 99), 9009);
    }

    #[test]
    fn test_three_digit_numbers() {
        assert_eq!(largest_palindrome_product(100, 999), 906609);
    }

    #[test]
    fn test_four_digit_numbers() {
        assert_eq!(largest_palindrome_product(1000, 9999), 9901 * 9999);
    }
}
