use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use std::collections::HashSet;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Clone, Eq, Hash, PartialEq)]
    enum Heading {
        North,
        South,
        East,
        West,
    }

    fn turn_right(h: Heading) -> Heading {
        match h {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }

    #[derive(Copy, Clone, Eq, Hash, PartialEq)]
    pub struct Coord {
        x: isize,
        y: isize,
    }

    impl Coord {
        pub fn new(x: isize, y: isize) -> Coord {
            Coord { x, y }
        }
        pub fn move_north(&self) ->Coord {Coord{x:self.x-1, y:self.y}}
        pub fn move_south(&self) ->Coord {Coord{x:self.x+1, y:self.y}}
        pub fn move_west(&self) ->Coord {Coord{x:self.x, y:self.y-1}}
        pub fn move_east(&self) ->Coord {Coord{x:self.x, y:self.y+1}}
    }

    fn parse<R: BufRead>(reader: R) -> Result<(HashSet<Coord>, Coord, isize, isize)> {
        let mut obstacles = HashSet::new();
        let mut guard = Coord::new(0, 0);
        let mut max_i = 0isize;
        let mut max_j = 0isize;
        for (i, line) in reader.lines().flatten().enumerate() {
            let i = i as isize;
            if i as isize > max_i {
                max_i = i as isize
            }
            for (j, c) in line.chars().enumerate() {
                let j = j as isize;
                if (j as isize > max_j) {
                    max_j = j as isize
                }
                match c {
                    '#' => {
                        let c = Coord::new(i,j);
                        obstacles.insert(c);
                    }
                    '^' => guard = Coord::new(i, j),
                    _ => (),
                }
            }
        }
        Ok((obstacles, guard, max_i, max_j))
    }

    fn move_guard(guard:Coord, heading:&Heading, obstacles:&HashSet<Coord>) -> (Coord, Heading) {
        match heading {
            Heading::North => {
                let next_pos = Coord::move_north(&guard);
                if obstacles.contains(&next_pos) {(guard, Heading::East)} else {(next_pos, Heading::North)}
            },
            Heading::East => {
                let next_pos = Coord::move_east(&guard);
                if obstacles.contains(&next_pos) {(guard, Heading::South)} else {(next_pos, Heading::East)}
            }
            Heading::South => {
                let next_pos = Coord::move_south(&guard);
                if obstacles.contains(&next_pos) {(guard, Heading::West)} else {(next_pos, Heading::South)}
            }
            Heading::West => {
                let next_pos = Coord::move_west(&guard);
                if obstacles.contains(&next_pos) {(guard, Heading::North)} else {(next_pos, Heading::West)}
            }
        }
    }
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let (obstacles, guard, max_i, max_j) = parse(reader).expect("unable to parse");
        let mut visited = HashSet::new();

        let mut guard = guard;
        let mut heading = Heading::North;
        while (guard.x>=0) & (guard.x <= max_i) & (guard.y >=0) & (guard.y <= max_j) {
            visited.insert(guard);
            (guard, heading) =  move_guard(guard,&heading, &obstacles);
        }

        // let answer = reader.lines().flatten().count();
        let answer = visited.len();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (obstacles, guard, max_i, max_j) = parse(reader).expect("unable to parse");
        let mut visited = HashSet::new();

        let mut guard = guard;
        let mut heading = Heading::North;
        while (guard.x>=0) & (guard.x <= max_i) & (guard.y >=0) & (guard.y <= max_j) {
            visited.insert((guard, heading.clone()));
            (guard, heading) =  move_guard(guard,&heading, &obstacles);
        }
        Ok(0)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
