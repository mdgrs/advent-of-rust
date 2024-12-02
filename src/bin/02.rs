use std::collections::HashSet;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R:BufRead>(reader: R) -> Vec<Vec<i64>> {
        let mut res = Vec::with_capacity(1000);
        for line in reader.lines().flatten() {
            let temp: Vec<i64> = line
                .split_whitespace()
                .map(|x| str::parse::<i64>(x).expect("unable to parse"))
                .collect();
            res.push(temp)
        }
        res
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let all_negative: HashSet<i64> = HashSet::from_iter(vec![-1i64, -2, -3].into_iter());
        let all_positive: HashSet<i64> = HashSet::from_iter(vec![1i64,2,3].into_iter());

        let mut res:usize = 0;
        for line in parse(reader) {
            let temp: HashSet<i64>= line.windows(2).map(|x| x[0]-x[1] ).collect();
            if temp.is_subset(&all_negative) | temp.is_subset(&all_positive) {
            res += 1;};
        }


        Ok(res)
    }
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let all_negative: HashSet<i64> = HashSet::from_iter(vec![-1i64, -2, -3].into_iter());
        let all_positive: HashSet<i64> = HashSet::from_iter(vec![1i64,2,3].into_iter());
        let mut res = 0;
        for line in parse(reader) {
            let mut temp_count:usize = 0;
            for i in 0..line.len() {
                let mut line = line.clone();
                line.remove(i);
                let temp: HashSet<i64>= line.windows(2).map(|x| x[0]-x[1] ).collect();
                if temp.is_subset(&all_negative) | temp.is_subset(&all_positive) {
                    temp_count = 1;
                }
            }
            res += temp_count;
            temp_count=0;
        }
        Ok(res)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
