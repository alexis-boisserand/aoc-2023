use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

fn main() {
    let sum: i32 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let sequences = (0..)
                .fold_while(vec![sequence; 1], |mut sequences, _| {
                    let diff: Vec<_> = sequences
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect();
                    if diff.iter().all(|&n| n == 0) {
                        Done(sequences)
                    } else {
                        sequences.push(diff);
                        Continue(sequences)
                    }
                })
                .into_inner();
            sequences
                .iter()
                .rev()
                .fold(0i32, |acc, sequence| acc + sequence.last().unwrap())
        })
        .sum();
    println!("{sum}");
}
