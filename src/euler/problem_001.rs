// Problem 1: Multiples of 3 and 5
//
//  If we list all the natural numbers below 10 that are multiples of 3 or 5, we get
//  3, 5, 6 and 9. The sum of these multiples is 23.
//
//  Find the sum for multiples of 3 and 5 below 1000.
pub fn problem_001() {
    let target_number = 1000;
    let result = sum_of_multiples(target_number);
    println!("The sum of multiples of 3 and 5 below {} is: {}", target_number, result);
}

fn sum_of_multiples(target: u32) -> u32 {
    let mut sum = 0;
    for num in 1..target {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn test_sum_of_multiples_0() {
        // The edge case of zero should return zero, as there are no numbers below zero.
        let target = 0;
        let expected = 0;
        assert_eq!(sum_of_multiples(target), expected);
    }

    #[test]
    fn test_sum_of_multiples_3() {
        // The edge case of three should return zero, as there are no multiples of 3 or 5 below three.
        let target = 3;
        let expected = 0;
        assert_eq!(sum_of_multiples(target), expected);
    }
    
    #[test]
    fn test_sum_of_multiples_5() {
        // The edge case of five should return 3, as the only multiple of 3 or 5 below five is 3.
        let target = 5;
        let expected = 3;
        assert_eq!(sum_of_multiples(target), expected);
    }
    #[test]
    fn test_sum_of_multiples_10() {
        let target = 10;
        let expected = 23;  // 3 + 5 + 6 + 9 = 23
        assert_eq!(sum_of_multiples(target), expected);
    }

    #[test]
    fn test_sum_of_multiples_1000() {
        let target = 1000;
        let expected = 233_168;
        assert_eq!(sum_of_multiples(target), expected);
    }


}
