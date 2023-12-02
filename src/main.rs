mod day01;
mod day02;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("day01") => day01::run(),
        Some("day02") => day02::run(),
        _ => println!("Invalid argument. Specify day to run"),
    }
}
