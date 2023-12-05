use std::collections::HashSet;

fn to_numbers(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.split(|ch: char| !ch.is_ascii_digit())
        .filter_map(|s| s.parse::<u32>().ok())
}

fn main() {
    let matches: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (winning, mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning: HashSet<_> = HashSet::from_iter(to_numbers(winning));
            to_numbers(mine)
                .filter_map(|n| winning.contains(&n).then_some(()))
                .count() as u32
        })
        .collect();

    let mut instances = vec![1u32; matches.len()];
    for i in 0..instances.len() {
        if matches[i] > 0 {
            for j in i + 1..=i + matches[i] as usize {
                instances[j] += instances[i];
            }
        }
    }

    let sum = instances.iter().sum::<u32>();
    println!("{sum}");
}
