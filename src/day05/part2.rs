pub fn solve(input: &str) -> usize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds: Vec<usize> = seeds
        .split_once(':')
        .expect("Input for seeds should be valid.")
        .1
        .split_ascii_whitespace()
        .map(|elem| {
            elem.parse::<usize>()
                .expect("Input for seeds should be valid.")
        })
        .collect();

    let mut seeds: Vec<(usize, usize)> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();

    let maps: Vec<Vec<(usize, usize, usize)>> = maps
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let line: Vec<usize> = line
                        .split_ascii_whitespace()
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

    for map in maps {
        let mut new = Vec::new();

        while let Some((start, end)) = seeds.pop() {
            let mut found = false;

            for &(dst, src, len) in map.iter() {
                let overlap_start = start.max(src);
                let overlap_end = end.min(src + len);

                if overlap_start < overlap_end {
                    found = true;

                    new.push((overlap_start - src + dst, overlap_end - src + dst));

                    if overlap_start > start {
                        seeds.push((start, overlap_start));
                    }

                    if end > overlap_end {
                        seeds.push((overlap_end, end));
                    }
                    break;
                }
            }

            if !found {
                new.push((start, end));
            }
        }

        seeds = new;
    }

    *seeds.iter().map(|(min, _)| min).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 46);
    }
}
