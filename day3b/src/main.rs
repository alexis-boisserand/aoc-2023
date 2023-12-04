use std::str::from_utf8;

fn adjacent_part_numbers(line: &str, index: usize) -> Vec<u32> {
    let line = line.as_bytes();
    let start = line[..=(if index == 0 { index } else { index - 1 })]
        .iter()
        .rev()
        .take_while(|ch| ch.is_ascii_digit())
        .count();
    let end = line[(if index == line.len() - 1 {
        index
    } else {
        index + 1
    })..]
        .iter()
        .take_while(|ch| ch.is_ascii_digit())
        .count();
    from_utf8(&line[index - start..=index + end])
        .unwrap()
        .split(|ch: char| !ch.is_ascii_digit())
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn gear_ratio(x: usize, y: usize, lines: &[&str]) -> Option<u32> {
    let start = if y == 0 { y } else { y - 1 };
    let end = if y == lines.len() - 1 { y } else { y + 1 };
    let part_numbers: Vec<u32> = (start..=end)
        .into_iter()
        .map(|y| adjacent_part_numbers(lines[y], x))
        .flatten()
        .collect();
    (part_numbers.len() == 2).then(|| part_numbers.iter().product())
}

fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let sum: u32 = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '*').then(|| (x, y)))
        })
        .flatten()
        .filter_map(|(x, y)| gear_ratio(x, y, &lines))
        .sum();
    println!("{sum}");
}
