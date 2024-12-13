use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    pub struct Equation {
        pub result: usize,
        pub operands: Vec<usize>
    }

    pub fn gather_possible_results(eq:&Equation) -> Vec<usize> {
        let mut res = vec![];
        let ( start,end) = eq.operands.split_first().unwrap();
        res.push(*start);
        for v in end.iter() {
            let mut copy1 = res.clone().iter().map(|x| x*v).collect_vec();
            let copy2 = res.clone().iter().map(|x| x+v).collect_vec();
            copy1.extend(copy2);
            res = copy1;
        }
        res
    }
    pub fn gather_possible_results2(eq:&Equation) -> Vec<usize> {
        let mut res = vec![];
        let ( start,end) = eq.operands.split_first().unwrap();
        res.push(*start);
        for v in end.iter() {
            let mut copy1 = res.clone().iter().map(|x| x*v).collect_vec();
            let copy2 = res.clone().iter().map(|x| x+v).collect_vec();
            let  copy3 = res.clone().iter().map(|x| format!("{x}{v}").parse::<usize>().unwrap()).collect_vec();
            copy1.extend(copy2);
            copy1.extend(copy3);
            res = copy1;
        }
        res
    }
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut res = Vec::new();
        for line in reader.lines().flatten() {
            let temp: Vec<String> = line.split_whitespace().map(|x|x.to_string()).collect();
            let (temp2, temp3) = temp.split_at(1);
            let result = temp2[0].trim_end_matches(":").parse::<usize>()?;
            let res2 = temp3.iter().map(|x| x.parse::<usize>().unwrap()).collect_vec();
            res.push(Equation {result, operands: res2})
        }

        let mut answer =0;
        for eq in res.iter() {
           let possible = gather_possible_results(&eq);
            if possible.iter().contains(&eq.result) {
               answer += eq.result;
            }
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut res = Vec::new();
        for line in reader.lines().flatten() {
            let temp: Vec<String> = line.split_whitespace().map(|x|x.to_string()).collect();
            let (temp2, temp3) = temp.split_at(1);
            let result = temp2[0].trim_end_matches(":").parse::<usize>()?;
            let res2 = temp3.iter().map(|x| x.parse::<usize>().unwrap()).collect_vec();
            res.push(Equation {result, operands: res2})
        }

        let mut answer =0;
        for eq in res.iter() {
            let possible = gather_possible_results2(&eq);
            if possible.iter().contains(&eq.result) {
                answer += eq.result;
            }
        }

        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
