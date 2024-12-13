use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use std::collections::{HashMap, HashSet};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    fn parse<R: BufRead>(reader: R) -> Result<(HashMap<char,HashSet<(isize,isize)>>, usize,usize )> {
        let mut res = HashMap::new();
        let mut max_i = 0;
        let mut max_j = 0;
        for (i,line) in reader.lines().flatten().enumerate() {
            max_i=i;
            max_j=line.len();
            for (j, ch) in line.chars().enumerate() {
                if ch != '.' {
                    res.entry(ch).and_modify(|x:&mut HashSet<(isize,isize)>| { x.insert((i as isize,j as isize)); }).or_insert(HashSet::from([(i as isize,j as isize)]));
                }

            }
        }
    Ok((res, max_i, max_j-1))
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut res = HashSet::new();
        let (positions, max_i, max_j) = parse(reader)?;
        let max_i = max_i as isize;
        let max_j = max_j as isize;
        for (ch, h) in positions {
            for first in h.clone() {
                for second in h.clone() {
                    if first != second {
                        let delta_i = second.0 - first.0;
                        let delta_j = second.1 - first.1;
                        let antinode = (second.0 + delta_i, second.1 + delta_j);
                        if (antinode.0>=0) & (antinode.0 <= max_i) & (antinode.1>=0) & (antinode.1<=max_j) {
                            res.insert(antinode);
                        }
                    }
                }
            }
        }
        Ok(res.len())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);
    let temp = part1(BufReader::new(TEST.as_bytes()))?;

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut res = HashSet::new();
        let (positions, max_i, max_j) = parse(reader)?;
        let max_i = max_i as isize;
        let max_j = max_j as isize;
        for (ch, h) in positions {
            for first in h.clone() {
                for second in h.clone() {
                    if first != second {
                        let delta_i = second.0 - first.0;
                        let delta_j = second.1 - first.1;
                        for i in 0..max_i {
                            let antinode = (second.0 + i*delta_i, second.1 + i*delta_j);
                            if (antinode.0>=0) & (antinode.0 <= max_i) & (antinode.1>=0) & (antinode.1<=max_j) {
                                res.insert(antinode);
                            }

                        }
                    }
                }
            }
        }
        Ok(res.len())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
