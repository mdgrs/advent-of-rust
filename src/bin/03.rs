use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"; // TODO: Add the test input
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).expect("problem");
        let re = Regex::new(r"(mul)\((?<one>\d{1,3}),(?<two>\d{1,3})\)").expect("problem2");
        let res: Vec<(usize, usize)> = re
            .captures_iter(&*input)
            .map(|x| {
                let one = x
                    .name("one")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .expect("bad parsing");
                let two = x
                    .name("two")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .expect("bad parsing");
                (one, two)
            })
            .collect();
        // TODO: Solve Part 1 of the puzzle
        let answer = res.iter().map(|(x, y)| x * y).sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).expect("problem");
        let re = Regex::new(r"(mul)\((?<one>\d{1,3}),(?<two>\d{1,3})\)").expect("problem2");
        let re_do = Regex::new(r"(do\(\))").expect("problem2");
        let re_dont = Regex::new(r"(don't\(\))").expect("problem2");
        let res = re
            .captures_iter(&*input)
            .map(|x| {
                let s = x.name("one").unwrap().start();
                let one = x
                    .name("one")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .expect("bad parsing");
                let two = x
                    .name("two")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .expect("bad parsing");
                (s, one * two)
            })
            .collect::<Vec<_>>();
        let res_do = re_do
            .find_iter(&*input)
            .map(|x| (x.start(), true))
            .collect::<Vec<_>>();
        let res_dont = re_dont
            .find_iter(&*input)
            .map(|x| (x.start(), false))
            .collect::<Vec<_>>();
        let activation = res_do
            .into_iter()
            .chain(res_dont.into_iter())
            .sorted()
            .collect::<Vec<_>>();

        let mut second_res = 0;
        for (index, product) in &res {
            let mut activated = true;
            for (start, state) in &activation {
                if start <= index {
                    activated = state.clone();
                }
            }
            if activated {
                second_res += product;
            }
        }

        Ok(second_res)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
