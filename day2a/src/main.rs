use std::convert::identity;

fn main() {
    let sum: u32 = include_str!("input.txt")
        .lines()
        .zip(1..)
        .filter_map(|(line, id)| {
            line.split_once(':')
                .unwrap()
                .1
                .split(|ch| ch == ';' || ch == ',')
                .map(|cube| {
                    let (value, color) = cube.trim().split_once(' ').unwrap();
                    value.parse::<u32>().unwrap()
                        <= match color {
                            "red" => 12,
                            "green" => 13,
                            "blue" => 14,
                            _ => unreachable!(),
                        }
                })
                .all(identity)
                .then(|| id)
        })
        .sum();
    println!("{sum}");
}
