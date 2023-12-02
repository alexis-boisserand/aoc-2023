fn first_digit(mut iter: impl Iterator<Item = char>) -> u32 {
    iter.find_map(|c| c.to_digit(10)).unwrap()
}


fn main() {
    let sum: u32 = include_str!("input.txt")
        .lines()
        .map(|line| first_digit(line.chars()) * 10 + first_digit(line.chars().rev()))
        .sum();
    println!("{sum}");
}
