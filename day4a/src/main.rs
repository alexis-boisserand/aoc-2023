use std::collections::HashSet;

fn main() {
    let sum: u32 = include_str!("input.txt")
        .lines()
        .filter_map(|line| {
            let (winning, mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let to_numbers = |s: &str| {
                s.split(|ch: char| !ch.is_ascii_digit())
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            };
            let winning: HashSet<_> = HashSet::from_iter(to_numbers(winning).into_iter());
            let matches = to_numbers(mine)
                .iter()
                .filter_map(|n| winning.contains(n).then_some(()))
                .count() as u32;
            (matches > 0).then(|| 2u32.pow(matches - 1))
        })
        .sum();
    println!("{sum}");
}
