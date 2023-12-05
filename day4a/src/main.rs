use std::collections::HashSet;

fn main() {
    let sum: u32 = include_str!("input.txt")
        .lines()
        .filter_map(|line| {
            let (winning, mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning: HashSet<_> = winning.split_whitespace().collect();
            let matches = mine.split_whitespace()
                .filter_map(|n| winning.contains(&n).then_some(()))
                .count() as u32;
            (matches > 0).then(|| 2u32.pow(matches - 1))
        })
        .sum();
    println!("{sum}");
}
