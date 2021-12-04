pub fn day3() {
    let input = include_str!("day3.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> usize {
    let counts = input
        .lines()
        .fold(BitSummary::default(), |counts, bits| counts.ingest(bits));
    let rates = counts.rates();
    rates.power()
}

fn part2(input: &str) -> usize {
    let oxy = part2_oxy(input);
    let co2 = part2_co2(input);
    oxy * co2
}

fn part2_oxy(input: &str) -> usize {
    part2_calc(input, |c| match c.zero.cmp(&c.one) {
        std::cmp::Ordering::Less => '1',
        std::cmp::Ordering::Equal => '1',
        std::cmp::Ordering::Greater => '0',
    })
}

fn part2_co2(input: &str) -> usize {
    part2_calc(input, |c| match c.zero.cmp(&c.one) {
        std::cmp::Ordering::Less => '0',
        std::cmp::Ordering::Equal => '0',
        std::cmp::Ordering::Greater => '1',
    })
}

fn part2_calc<F: Fn(&BitCount) -> char>(input: &str, calc: F) -> usize {
    let mut lines: Vec<_> = input.lines().collect();
    let mut bit_index = 0;
    while lines.len() > 1 {
        let count = lines
            .iter()
            .map(|s| s.chars().nth(bit_index).unwrap())
            .fold(BitCount::default(), |mut c, b| {
                c.ingest(b);
                c
            });
        lines.retain(|line| line.chars().nth(bit_index).unwrap() == calc(&count));
        bit_index += 1;
    }

    usize::from_str_radix(lines[0], 2).unwrap()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
struct BitCount {
    zero: usize,
    one: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
struct BitSummary {
    counts: Vec<BitCount>,
}

impl BitSummary {
    pub fn ingest(mut self, bits: &str) -> Self {
        let bits: Vec<_> = bits.chars().collect();
        if self.counts.is_empty() {
            self.counts.resize(bits.len(), BitCount::default());
        } else {
            assert_eq!(bits.len(), self.counts.len(), "Got differing length bits");
        }

        for (count, bit) in self.counts.iter_mut().zip(bits.into_iter()) {
            count.ingest(bit);
        }

        self
    }

    pub fn gamma(&self) -> usize {
        let bits: String = self.counts.iter().map(|b| b.most_common()).collect();
        usize::from_str_radix(&bits, 2).unwrap()
    }

    pub fn epsilon(&self) -> usize {
        let bits: String = self.counts.iter().map(|b| b.least_common()).collect();
        usize::from_str_radix(&bits, 2).unwrap()
    }

    pub fn rates(&self) -> Rates {
        Rates {
            gamma: self.gamma(),
            epsilon: self.epsilon(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
struct Rates {
    // result of most common bits for each position
    gamma: usize,
    // result of least common bits for each position
    epsilon: usize,
}

impl BitCount {
    pub fn ingest(&mut self, bit: char) {
        match bit {
            '0' => self.zero += 1,
            '1' => self.one += 1,
            _ => panic!("Got {}", bit),
        }
    }

    pub fn most_common(&self) -> char {
        match self.zero.cmp(&self.one) {
            std::cmp::Ordering::Less => '0',
            std::cmp::Ordering::Equal => todo!(),
            std::cmp::Ordering::Greater => '1',
        }
    }

    pub fn least_common(&self) -> char {
        match self.most_common() {
            '0' => '1',
            '1' => '0',
            _ => panic!("Bad bit"),
        }
    }
}

impl Rates {
    pub fn power(&self) -> usize {
        self.gamma * self.epsilon
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(part1(input), 198);
    }

    #[test]
    fn test_part2() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(part2(input), 230);
    }

    #[test]
    fn test_part2_co2() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let co2 = part2_co2(input);
        assert_eq!(co2, 10);
    }
}
