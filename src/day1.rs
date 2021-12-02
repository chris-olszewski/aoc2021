use std::io::BufRead;

#[derive(Default)]
struct State {
    increases: usize,
    previous: Option<usize>,
}

impl State {
    fn add(mut self, depth: usize) -> Self {
        if let Some(prev) = self.previous {
            if prev < depth {
                self.increases += 1;
            }
        }
        self.previous = Some(depth);
        self
    }
}

pub fn day1()  {
    println!("{}", calculate_inc_part1(include_str!("day1.txt")));
    println!("{}", calculate_inc_part2(include_str!("day1.txt")));
}

fn calculate_inc_part1(input: &str) -> usize {
    let depths = parse_input(input);
    let result = depths
        .into_iter()
        .fold(State::default(), |acc, d| acc.add(d));
    result.increases
}

fn calculate_inc_part2(input: &str) -> usize {
    let depths = parse_input(input);
    let result = depths
        .windows(3)
        .into_iter()
        .fold(State::default(), |acc, d| acc.add(d.iter().sum()));
    result.increases
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .as_bytes()
        .lines()
        .map(|num| num.unwrap().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data() {
        let test_input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(calculate_inc_part1(test_input), 7);
    }

    #[test]
    fn test_data_part2() {
        let test_input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(calculate_inc_part2(test_input), 5);
    }
}
