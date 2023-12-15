fn main() {
    let line = include_str!("input.txt").lines().next().unwrap();
    let sum: u32 = line
        .split(",")
        .map(|s| {
            s.chars()
                .fold(0u32, |value, c| ((value + u32::from(c)) * 17) % 256)
        })
        .sum();
    println!("{sum}");
}
