mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("day01") => day01::run(),
        Some("day02") => day02::run(),
        Some("day03") => day03::run(),
        Some("day04") => day04::run(),
        Some("day05") => day05::run(),
        Some("day06") => day06::run(),
        Some("day07") => day07::run(),
        Some("day08") => day08::run(),
        Some("day09") => day09::run(),
        Some("day10") => day10::run(),
        Some("day11") => day11::run(),
        Some("day12") => day12::run(),
        Some("day13") => day13::run(),
        Some("day14") => day14::run(),
        Some("day15") => day15::run(),
        Some("day16") => day16::run(),
        Some("day17") => day17::run(),
        Some("day18") => day18::run(),
        Some("day19") => day19::run(),
        Some("day20") => day20::run(),
        Some("day21") => day21::run(),
        Some("day22") => day22::run(),
        Some("day23") => day23::run(),
        Some("day24") => day24::run(),
        Some("day25") => day25::run(),
        Some("all") => {
            println!("Day 1:");
            day01::run();
            println!("Day 2:");
            day02::run();
            println!("Day 3:");
            day03::run();
            println!("Day 4:");
            day04::run();
            println!("Day 5:");
            day05::run();
            println!("Day 6:");
            day06::run();
            println!("Day 7:");
            day07::run();
            println!("Day 8:");
            day08::run();
            println!("Day 9:");
            day09::run();
            println!("Day 10:");
            day10::run();
            println!("Day 11:");
            day11::run();
            println!("Day 12:");
            day12::run();
            println!("Day 13:");
            day13::run();
            println!("Day 14:");
            day14::run();
            println!("Day 15:");
            day15::run();
            println!("Day 16:");
            day16::run();
            println!("Day 17:");
            day17::run();
            println!("Day 18:");
            day18::run();
            println!("Day 19:");
            day19::run();
            println!("Day 20:");
            day20::run();
            println!("Day 21:");
            day21::run();
            println!("Day 22:");
            day22::run();
            println!("Day 23:");
            day23::run();
            println!("Day 24:");
            day24::run();
            println!("Day 25:");
            day25::run();
            println!("Finito!");
        }
        _ => println!("Invalid argument. Specify day to run"),
    }
}
