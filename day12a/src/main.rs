use itertools::{repeat_n, Itertools};
use Condition::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => unreachable!(),
        }
    }
}

fn calculate_contiguous(individual: &[Condition]) -> Vec<(u32, Condition)> {
    let mut contiguous = Vec::new();
    let mut current = &individual[0];
    let mut count = 1;
    for c in &individual[1..] {
        if *c == *current {
            count += 1;
        } else {
            contiguous.push((count, *current));
            count = 1;
            current = c;
        }
    }

    contiguous.push((count, *current));
    contiguous
}

fn expand<T: Clone>(individual: Vec<T>) -> Vec<T> {
    repeat_n(individual, 5).into_iter().flatten().collect()
}

fn main() {
    let sum: u32 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (individual, contiguous) = line.split_once(' ').unwrap();
            (
                individual
                    .chars()
                    .map(|c| Condition::from(c))
                    .collect::<Vec<_>>(),
                contiguous
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(individual, contiguous)| {
            //let individual = expand(individual);
            //let contiguous = expand(contiguous);
            let mut count = 0;
            let partial_contiguous = calculate_contiguous(&individual);
            let unknowns = individual.iter().filter(|&c| *c == Unknown).count();
            println!("{partial_contiguous:?} {contiguous:?} {unknowns}");
            //let perms = repeat_n(vec![Operational, Damaged], unknowns).multi_cartesian_product();
            //for perm in perms {
            //    let mut test = individual.clone();
            //    test.iter_mut()
            //        .filter(|c| **c == Unknown)
            //        .zip(perm.iter())
            //        .for_each(|(dst, src)| *dst = *src);
            //    let test_contiguous = calculate_contiguous(&test);
            //    if test_contiguous == contiguous {
            //        count += 1;
            //    }
            //}
            count
        })
        .sum();
    println!("{sum}");
}
