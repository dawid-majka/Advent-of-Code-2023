mod day01;
mod day02;
mod day03;
mod day04;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day13;
mod day14;
mod day15;
mod day16;
mod day18;
mod day21;
mod day22;
mod day23;
mod day24;

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
        Some("day10") => day10::run(),
        Some("day11") => day11::run(),
        Some("day13") => day13::run(),
        Some("day14") => day14::run(),
        Some("day15") => day15::run(),
        Some("day16") => day16::run(),
        Some("day18") => day18::run(),
        Some("day21") => day21::run(),
        Some("day22") => day22::run(),
        Some("day23") => day23::run(),
        Some("day24") => day24::run(),
        _ => println!("Invalid argument. Specify day to run"),
    }
}
