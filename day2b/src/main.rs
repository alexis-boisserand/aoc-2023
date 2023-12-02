use core::cmp::max;

fn main() {
    let sum: u32 = include_str!("input.txt")
        .lines()
        .map(
            line.split_once(':')
                .unwrap()
                .1
                .split(|ch| ch == ',' || ch == ';')
                .fold([0, 0, 0], |mut acc, cube| {
                    let (value, color) = cube.trim().split_once(' ').unwrap();
                    let index = match color {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => unreachable!(),
                    };
                    acc[index] = max(acc[index], value.parse().unwrap());
                    acc
                })
                .iter()
                .product(),
        )
        .sum();
    println!("{sum}");
}
