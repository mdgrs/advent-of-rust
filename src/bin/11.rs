use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;

const DAY: &str = "11"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(mut input: R) -> Vec<usize> {
        let mut res = "".to_string();
       input.read_to_string(&mut res).expect("could not read input file");
        res.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect_vec()
    }

    fn one_iter(input:Vec<usize>) -> Vec<usize> {
        let mut res = vec![];
        for item in input {
            if item==0 {
                res.push(1);
            }
            else {
                let l = item.to_string().chars().count();
                if l %2 ==0 {
                    let mid = l/2;
                    let first:String = item.to_string().chars().take(mid).collect();
                    let second: String = item.to_string().chars().skip(mid).collect();
                    res.push(first.parse::<usize>().unwrap());
                    res.push(second.parse::<usize>().unwrap());
                }
                else {
                    res.push(item*2024)
                }

            }
            }
        res
        }
    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let input = parse(reader);

        let mut res = input;
        for _i in 0..25 {
            res = one_iter(res);
        }
        let answer =res.len();
        Ok(answer)

    }

    // TODO: Set the expected answer for the test input
    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
