use std::collections::HashSet;

fn main() {
    let matches: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (winning, mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning: HashSet<_> = winning.split_whitespace().collect();
            mine.split_whitespace()
                .filter_map(|n| winning.contains(&n).then_some(()))
                .count() as u32
        })
        .collect();

    let mut instances = vec![1u32; matches.len()];
    (0..instances.len()).for_each(|i| {
        (i + 1..=i + matches[i] as usize).for_each(|j| instances[j] += instances[i]);
    });

    let sum = instances.iter().sum::<u32>();
    println!("{sum}");
}
