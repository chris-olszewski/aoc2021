use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<_> = env::args().collect();
    let day = args[1].parse::<usize>().expect("Unable to convert day to int");
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        _ => todo!()
    }
}
