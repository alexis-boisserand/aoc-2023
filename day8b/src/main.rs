use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let format = lines.next().unwrap();
    let network: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|line| {
            let (src, nodes) = line.split_once('=').unwrap();
            let src = src.trim_end();
            let trims: &[_] = &[' ', '(', ')'];
            let (left, right) = nodes.trim_matches(trims).split_once(',').unwrap();
            let right = right.trim_start();
            (src, (left, right))
        })
        .collect();

    let counts: Vec<_> = network
        .keys()
        .filter(|node| node.ends_with('A'))
        .into_iter()
        .map(|node| {
            let (_, count) = format
                .chars()
                .cycle()
                .fold_while((node, 0u64), |(src, count), dir| {
                    let (left, right) = network.get(src).unwrap();
                    let dst = match dir {
                        'L' => left,
                        'R' => right,
                        _ => unreachable!(),
                    };
                    if !dst.ends_with('Z') {
                        Continue((dst, count + 1))
                    } else {
                        Done((dst, count + 1))
                    }
                })
                .into_inner();
            count
        })
        .collect();

    let count = counts.into_iter().fold(1u64, |a, b| lcm(a, b));
    println!("{count}");
}
