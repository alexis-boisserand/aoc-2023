use std::collections::HashSet;

fn to_numbers(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.split(|ch: char| !ch.is_ascii_digit())
        .filter_map(|s| s.parse::<u32>().ok())
}

fn main() {
    let sum: u32 = include_str!("input.txt")
        .lines()
        .filter_map(|line| {
            let (winning, mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning: HashSet<_> = HashSet::from_iter(to_numbers(winning));
            let matches = to_numbers(mine)
                .filter_map(|n| winning.contains(&n).then_some(()))
                .count() as u32;
            (matches > 0).then(|| 2u32.pow(matches - 1))
        })
        .sum();
    println!("{sum}");
}
