use std::collections::HashSet;

fn to_numbers(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.split(|ch: char| !ch.is_ascii_digit())
        .filter_map(|s| s.parse::<u32>().ok())
}

fn count_copies(index: usize, matches: &[u32], instances: &mut [u32]) {
    instances[index] += 1;
    if matches[index] > 0 {
        (index + 1..=index + matches[index] as usize)
            .for_each(|i| count_copies(i, matches, instances));
    }
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

    let mut instances = vec![0u32; matches.len()];
    for index in 0..instances.len() {
        count_copies(index, &matches, &mut instances)
    }

    let sum = instances.iter().sum::<u32>();
    println!("{sum}");
}
