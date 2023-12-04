use std::str::from_utf8;

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
        .filter_map(|(x, y)| {
            let part_numbers: Vec<_> = ((if y == 0 { y } else { y - 1 })
                ..=(if y == lines.len() - 1 { y } else { y + 1 }))
                .into_iter()
                .map(|y| {
                    let line = lines[y].as_bytes();
                    let start = line[..=(if x == 0 { x } else { x - 1 })]
                        .iter()
                        .rev()
                        .take_while(|ch| ch.is_ascii_digit())
                        .count();
                    let end = line[(if x == line.len() - 1 { x } else { x + 1 })..]
                        .iter()
                        .take_while(|ch| ch.is_ascii_digit())
                        .count();
                    from_utf8(&line[x - start..=x + end])
                        .unwrap()
                        .split(|ch: char| !ch.is_ascii_digit())
                        .filter_map(|s| s.parse::<u32>().ok())
                })
                .flatten()
                .collect();
            (part_numbers.len() == 2).then(|| part_numbers.iter().product::<u32>())
        })
        .sum();
    println!("{sum}");
}
