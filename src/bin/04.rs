use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use std::cmp::min;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>>{
        // let res = reader.lines().flatten().map(|x| x.chars().collect()).collect();
        let mut res = Vec::new();
        for line in reader.lines().flatten() {
            let mut temp = Vec::new();
            for c in line.chars() {
                temp.push(c.clone());
            }
            res.push(temp);
        }
        Ok(res)
    }

    fn transpose(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let nrows = board.len();
        let ncols = board[0].len();
        let mut res = vec![vec![]; ncols];
        for i in 0..nrows {
            for j in 0..ncols {
                res[j].push(board[i][j]);
            }
        }
        res
    }
    fn get_other_diag(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut res = Vec::new();
        let nrows = board.len();
        let ncols = board[0].len();
        let mut starters = vec![];
        for i in 0..nrows{
            starters.push((i,0))
        }
        for j in 1..ncols{
            starters.push((0, j))
        }
        for starter in starters {
            let size = min(nrows-starter.0-1, ncols-starter.1-1);
            if size > 2 {
                let mut temp = vec![];
                for i in 0..=size {
                    let value = board[starter.0 + i][starter.1 + i];
                    temp.push(value);
                }
                res.push(temp);
            }
        }
        res
    }

    fn get_diag(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut res = Vec::new();
        let nrows = board.len();
        let ncols = board[0].len();
        let mut starters = vec![];
        for i in 0..nrows{
            starters.push((i,0))
        }
        for j in 1..ncols{
            starters.push((nrows-1, j))
        }
        for starter in starters {
            let size = min(starter.0, ncols-starter.1-1);
            if size > 2 {
                let mut temp = vec![];
                for i in 0..=size {
                    let value = board[starter.0 - i][starter.1 + i];
                    temp.push(value);
                }
                res.push(temp);
            }
        }
        res
    }

    fn count_occurences(row:&Vec<char>) -> usize {
        let mut res = 0;
        let a = row.windows(4).filter(|x| x.iter().zip("XMAS".chars()).all(|(a,b)| a == &b)).count();
        let b = row.windows(4).filter(|x| x.iter().zip("SAMX".chars()).all(|(a,b)| a == &b)).count();
        a+b
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut res = 0;
        let board = parse(reader).expect("bad parsing");
        res += board.iter().map(count_occurences).sum::<usize>();
        let diag = get_diag(&board);
        res += diag.iter().map(count_occurences).sum::<usize>();
        let other_diag = get_other_diag(&board);
        res += other_diag.iter().map(count_occurences).sum::<usize>();
        let transposed = transpose(&board);
        res += transposed.iter().map(count_occurences).sum::<usize>();

        Ok(res)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn diag(i:usize, j:usize,board: &Vec<Vec<char>>) -> bool {
        let positive = if (board[i-1][j+1] == 'S') & (board[i+1][j-1] == 'M') {
            true
        } else if (board[i-1][j+1] == 'M') & (board[i+1][j-1] == 'S') {true}
        else {false};
        let negative = if (board[i-1][j-1] == 'S') & (board[i+1][j+1] == 'M') {
            true
        } else if (board[i-1][j-1] == 'M') & (board[i+1][j+1] == 'S') {true}
        else {false};
        positive & negative
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut res =0;
        let board = parse(reader).expect("bad parsing");
        let mut a_indices = vec![];
        for i in 1..board.len()-1 {
            for j in 1..board[0].len()-1 {
                if board[i][j] == 'A' {
                    a_indices.push((i,j));
                }
            }
        }
        for (i,j) in a_indices {
            if diag(i,j,&board) {res+=1}
        }

        Ok(res)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
