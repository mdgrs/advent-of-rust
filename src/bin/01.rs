use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> (BinaryHeap<u64>, BinaryHeap<u64>) {
        // TODO: Solve Part 1 of the puzzle
        // let answer = reader.lines().flatten().count();
        let mut left = std::collections::BinaryHeap::with_capacity(1000);
        let mut right = std::collections::BinaryHeap::with_capacity(1000);
        for line in reader.lines().flatten() {
            let temp: Vec<u64> = line
                .split_whitespace()
                .map(|x| str::parse::<u64>(x).expect("unable to parse"))
                .collect();

            left.push(temp[0]);
            right.push(temp[1]);
        }
        (left, right)
    }
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        // let answer = reader.lines().flatten().count();
        let (left, right) = parse(reader);
        let left_input = left.into_sorted_vec();
        let right_input = right.into_sorted_vec();
        let mut res: u64 = 0;
        for (i, j) in left_input.into_iter().zip(right_input.into_iter()) {
            res += i.abs_diff(j);
        }
        Ok(res as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut res = 0u64;
        let (mut left, mut right) = parse(reader);
        let mut prev_value = 0u64;
        let mut count = 0;
        let mut right_map = HashMap::new();
        while let Some(y) = right.pop() {
            if y == prev_value {
                count += 1;
                if right.is_empty() {
                    right_map.insert(prev_value, count);
                }
            } else {
                right_map.insert(prev_value, count);
                count = 1;
                prev_value = y;
            }
        }
        // println!("right_map = {:#?}", right_map);
        while let Some(x) = left.pop() {
            if let Some(y) = right_map.get(&x) {
                res += x * y;
            }
        }
        Ok(res as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
