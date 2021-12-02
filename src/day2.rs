use std::ops::Add;
use std::str::FromStr;

pub fn day2() {
    let input = include_str!("day2.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> usize {
    let pos = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .fold(Position::default(), |pos, d| pos + d);
    pos.product()
}

fn part2(input: &str) -> usize {
    let pos = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .fold(PositionAim::default(), |pos, d| pos + d);
    pos.product()
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    depth: usize,
    horizontal: usize,
}

impl Position {
    pub fn product(&self) -> usize {
        self.depth * self.horizontal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Down(usize),
    Up(usize),
    Forward(usize),
}

impl Add<Instruction> for Position {
    type Output = Position;

    fn add(mut self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction::Down(x) => self.depth += x,
            Instruction::Up(x) => self.depth -= x,
            Instruction::Forward(x) => self.horizontal += x,
        }
        self
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let direction = parts.next().expect("No direction");
        let distance: usize = parts
            .next()
            .expect("No distance")
            .parse()
            .expect("Unable to convert distance to int");

        match direction {
            "down" => Ok(Instruction::Down(distance)),
            "up" => Ok(Instruction::Up(distance)),
            "forward" => Ok(Instruction::Forward(distance)),
            _ => panic!("Unrecognized direction '{}'", direction),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PositionAim {
    depth: usize,
    horizontal: usize,
    aim: usize,
}

impl PositionAim {
    pub fn product(&self) -> usize {
        self.depth * self.horizontal
    }
}

impl Add<Instruction> for PositionAim {
    type Output = PositionAim;

    fn add(mut self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction::Down(x) => self.aim += x,
            Instruction::Up(x) => self.aim -= x,
            Instruction::Forward(x) => {
                self.horizontal += x;
                self.depth += self.aim * x;
            },
        }
        self
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(part1(input), 150);
    }

    #[test]
    fn test_part2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(part2(input), 900);
    }
}