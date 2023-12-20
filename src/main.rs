mod day01;
mod day02;
mod day03;
mod day04;
mod day06;
mod day07;
mod day08;
mod day09;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("day01") => day01::run(),
        Some("day02") => day02::run(),
        Some("day03") => day03::run(),
        Some("day04") => day04::run(),
        Some("day06") => day06::run(),
        Some("day07") => day07::run(),
        Some("day08") => day08::run(),
        Some("day09") => day09::run(),
        Some("day13") => day13::run(),
        Some("day14") => day14::run(),
        Some("day15") => day15::run(),
        Some("day16") => day16::run(),
        _ => println!("Invalid argument. Specify day to run"),
    }
}
