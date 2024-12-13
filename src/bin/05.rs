use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use std::collections::HashMap;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const INPUT_FILE2: &str = concatcp!("input/", DAY, "_bis.txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
";

const TEST2: &str = "\
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, reader2: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let rules = reader
            .lines()
            .flatten()
            .map(|x| {
                let temp = x.split("|").collect_vec();
                let r1 = temp[0].parse::<usize>().expect("test");
                let r2 = temp[1].parse::<usize>().expect("test");
                (r1, r2)
            })
            .collect_vec();
        let mut res = 0;
        let updates = reader2
            .lines()
            .flatten()
            .map(|x| {
                x.split(",")
                    .map(|x| x.parse::<usize>().expect("test"))
                    .collect_vec()
            })
            .collect_vec();

        for update in updates {
            let mut hash = HashMap::new();
            for value in update.iter().enumerate() {
                hash.insert(value.1, value.0);
            }
            let b = rules.iter().all(|(r1, r2)| {
                let first = hash.get(r1);
                let second = hash.get(r2);
                if let (Some(f), Some(s)) = (first, second) {
                    f < s
                } else {
                    true
                }

            });
            let mut bad_updates = vec![];
            if b {
                let t = (update.len()-1).checked_div(2).unwrap();
                res+= update[t];
               }
            else {bad_updates.push(update);}
        }

        Ok(res)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(
        143,
        part1(
            BufReader::new(TEST.as_bytes()),
            BufReader::new(TEST2.as_bytes())
        )?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let input_file2 = BufReader::new(File::open(INPUT_FILE2)?);
    let result = time_snippet!(part1(input_file, input_file2)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let rules = reader
            .lines()
            .flatten()
            .map(|x| {
                let temp = x.split("|").collect_vec();
                let r1 = temp[0].parse::<usize>().expect("test");
                let r2 = temp[1].parse::<usize>().expect("test");
                (r1, r2)
            })
            .collect_vec();
        let mut res = 0;
        let updates = reader2
            .lines()
            .flatten()
            .map(|x| {
                x.split(",")
                    .map(|x| x.parse::<usize>().expect("test"))
                    .collect_vec()
            })
            .collect_vec();
        Ok(0)
    }

    assert_eq!(, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
