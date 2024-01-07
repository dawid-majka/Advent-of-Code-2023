pub fn solve(input: &str) -> usize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let mut seeds: Vec<usize> = seeds
        .split_once(':')
        .expect("Input for seeds should be valid.")
        .1
        .split_ascii_whitespace()
        .map(|elem| {
            elem.parse::<usize>()
                .expect("Input for seeds should be valid.")
        })
        .collect();

    let maps: Vec<Vec<(usize, usize, usize)>> = maps
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let line: Vec<&str> = line.split_ascii_whitespace().collect();
                    let line: Vec<usize> = line
                        .iter()
                        .map(|elem| {
                            elem.parse::<usize>()
                                .expect("Input for maps should be valid.")
                        })
                        .collect();
                    (line[0], line[1], line[2])
                })
                .collect::<Vec<(usize, usize, usize)>>()
        })
        .collect();

    *seeds
        .iter_mut()
        .map(|seed: &mut usize| {
            for map in maps.iter() {
                for &(dst, src, len) in map.iter() {
                    if (src..(src + len)).contains(seed) {
                        *seed = dst + (*seed - src);
                        break;
                    }
                }
            }

            seed
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 35);
    }
}
