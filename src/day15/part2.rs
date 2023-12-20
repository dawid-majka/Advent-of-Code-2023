pub fn solve(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    input.split(',').for_each(|step| {
        if let Some((label, f_length)) = step.split_once('=') {
            let hash = label
                .chars()
                .fold(0, |acc, c| (acc + c as usize) * 17 % 256);
            let f_length = f_length
                .parse::<usize>()
                .expect("Focal length to be valid number");

            if let Some(container) = boxes.get_mut(hash) {
                if let Some(position) = container.iter().position(|&(l, _)| l == label) {
                    container[position] = (label, f_length)
                } else {
                    container.push((label, f_length));
                }
            }
        } else if let Some((label, _)) = step.split_once('-') {
            let hash = label.chars().fold(0, |acc, c| (acc + c as u32) * 17 % 256);

            if let Some(container) = boxes.get_mut(hash as usize) {
                if let Some(position) = container.iter().position(|&(l, _)| l == label) {
                    container.remove(position);
                }
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, container)| {
            container
                .iter()
                .enumerate()
                .map(|(elem_idx, elem)| (box_idx + 1) * (elem_idx + 1) * elem.1)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle_full() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 145);
    }
}
