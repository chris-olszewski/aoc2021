use std::str::FromStr;

use itertools::Itertools;

pub fn day4() {
    let input = include_str!("day4.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> usize {
    let Input {
        numbers,
        mut boards,
    } = Input::from_str(input).unwrap();
    for number in numbers {
        for boards in boards.iter_mut() {
            // what about ties :)
            if let MarkResult::Win = boards.mark(number) {
                return boards.score();
            }
        }
    }

    panic!("No winner");
}

fn part2(input: &str) -> usize {
    let Input {
        numbers,
        mut boards,
    } = Input::from_str(input).unwrap();
    for number in numbers {
        match boards.len() == 1 {
            false => {
                boards = boards
                    .into_iter()
                    .filter_map(|mut b| match b.mark(number) {
                        MarkResult::Win => None,
                        MarkResult::Continue => Some(b),
                    })
                    .collect();
            }
            true => {
                if let MarkResult::Win = boards[0].mark(number) {
                    break;
                }
            }
        }
    }

    boards[0].score()
}

struct Input {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

struct Board {
    marked: Vec<(usize, usize)>,
    grid: [[usize; 5]; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MarkResult {
    Win,
    Continue,
}

impl Board {
    pub fn mark(&mut self, number: usize) -> MarkResult {
        let mut newly_marked = Vec::new();
        for (row, col) in (0..5).cartesian_product(0..5) {
            let cell = self.grid[row][col];
            if cell == number {
                newly_marked.push((row, col));
            }
        }

        // pretty sure this will only ever be 1
        assert!(newly_marked.len() <= 1);
        for marked in newly_marked {
            self.marked.push(marked);
            if self.is_row_complete(marked.0) || self.is_column_complete(marked.1) {
                return MarkResult::Win;
            }
        }

        MarkResult::Continue
    }

    fn is_row_complete(&self, row: usize) -> bool {
        assert!((0..5).contains(&row), "Row out of bounds");
        self.marked.iter().filter(|(r, _)| *r == row).count() == 5
    }

    fn is_column_complete(&self, column: usize) -> bool {
        assert!((0..5).contains(&column), "Row out of bounds");
        self.marked.iter().filter(|(_, c)| *c == column).count() == 5
    }

    fn lookup(&self, (row, col): (usize, usize)) -> usize {
        self.grid[row][col]
    }

    pub fn score(&self) -> usize {
        let last_number = self.lookup(*self.marked.last().unwrap());
        let unmarked_numbers: usize = (0..5)
            .cartesian_product(0..5)
            .filter(|pos| !self.marked.contains(pos))
            .map(|pos| self.lookup(pos))
            .sum();
        unmarked_numbers * last_number
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let numbers: Vec<_> = lines
            .next()
            .expect("No first line")
            .split(',')
            .map(|n| usize::from_str(n).unwrap())
            .collect();
        let boards = lines
            .chunks(6)
            .into_iter()
            .map(|chunk| {
                let mut rows = [[0; 5]; 5];
                chunk
                    .dropping(1)
                    .map(|line| {
                        let mut row = [0; 5];
                        line.split_ascii_whitespace()
                            .map(|n| usize::from_str(n).unwrap())
                            .enumerate()
                            .for_each(|(i, x)| row[i] = x);
                        row
                    })
                    .enumerate()
                    .for_each(|(i, r)| rows[i] = r);

                Board {
                    grid: rows,
                    marked: Vec::new(),
                }
            })
            .collect();

        Ok(Input { numbers, boards })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
    
 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6
    
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1924);
    }
}
