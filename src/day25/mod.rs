mod part1;

pub fn run() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1::solve(input));
}
