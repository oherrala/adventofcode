//! Advent of Code 2017 day 1 part 2 <http://adventofcode.com/2017/day/1>
//! 

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdio");

    println!("Got digits: {}", input);
    println!("Sum of digits: {}", sum_of_digits(&input));
}

fn sum_of_digits(digits: &str) -> u32 {
    let digits: Vec<u32> = digits.trim().chars().map(|c| c.to_digit(10).expect("to_digit")).collect();
    assert_eq!(digits.len() % 2, 0, "Fortunately, your list has an even number of elements.");

    let lower = digits.iter().take(digits.len()/2);
    let upper = digits.iter().skip(digits.len()/2);
    let zipper = lower.zip(upper);
    zipper.filter(|&(a,b)| a == b).map(|(a,_)| a*2).sum()
}

#[cfg(test)]
mod tests_01p2 {
    use super::*;

    // 1212 produces 6: the list contains 4 items, and all four digits match the
    // digit 2 items ahead.
    #[test]
    fn sum_of_digits1() {
        let digits = "1212";
        assert_eq!(sum_of_digits(digits), 6);
    }

    // 1221 produces 0, because every comparison is between a 1 and a 2.
    #[test]
    fn sum_of_digits2() {
        let digits = "1221";
        assert_eq!(sum_of_digits(digits), 0);
    }

    // 123425 produces 4, because both 2s match each other, but no other digit
    // has a match.
    #[test]
    fn sum_of_digits3() {
        let digits = "123425";
        assert_eq!(sum_of_digits(digits), 4);
    }

    // 123123 produces 12.
    #[test]
    fn sum_of_digits4() {
        let digits = "123123";
        assert_eq!(sum_of_digits(digits), 12);
    }

    // 12131415 produces 4.
    #[test]
    fn sum_of_digits5() {
        let digits = "12131415";
        assert_eq!(sum_of_digits(digits), 4);
    }
}
