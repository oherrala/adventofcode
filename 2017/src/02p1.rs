//! Advent of Code 2017 day 2 part 1 <http://adventofcode.com/2017/day/2>
//!

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdio");

    let matrix = read_input(&input);
    println!("matrix is\n{:?}", matrix);
    println!("Checksum is {}", checksum(&matrix));
}

fn checksum(matrix: &[Vec<usize>]) -> usize {
    let mut sum = 0;
    for row in matrix {
        let cs = row_checksum(row);
        sum += cs;
    }
    sum
}

fn row_checksum(row: &[usize]) -> usize {
    eprintln!("Row {:?}", row);
    let min = row.iter().min().expect("min value");
    let max = row.iter().max().expect("max value");
    max - min
}

fn read_input(input: &str) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    let lines = input.lines();

    for line in lines {
        if line.trim().len() < 3 {
            continue
        }

        let chars = line.trim().split_whitespace();
        let digits: Vec<usize> = chars.map(|c| c.parse::<usize>().expect("parse") as usize).collect();
        ret.push(digits);
    }

    ret
}


#[cfg(test)]
mod tests_02p1 {
    use super::*;

    // 5 1 9 5
    // 7 5 3
    // 2 4 6 8

    #[test]
    fn row_checksum1() {
        let row = vec![5, 1, 9, 5];
        assert_eq!(row_checksum(&row), 8);
    }

    #[test]
    fn row_checksum2() {
        let row = vec![7, 5, 3];
        assert_eq!(row_checksum(&row), 4);
    }

    #[test]
    fn row_checksum3() {
        let row = vec![2, 4, 6, 8];
        assert_eq!(row_checksum(&row), 6);
    }

    #[test]
    fn read_matrix() {
        let input = r#"
            5 1 9 5
            7 5 3
            2 4 6 8
        "#;

        let matrix = read_input(input);
        let mut iter = matrix.iter();
        assert_eq!(iter.next(), Some(&vec![5, 1, 9, 5]));
        assert_eq!(iter.next(), Some(&vec![7, 5, 3]));
        assert_eq!(iter.next(), Some(&vec![2, 4, 6, 8]));
    }
}
