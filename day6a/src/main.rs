fn main() {
    let mut lines: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
        })
        .collect();

    let distances = lines.pop().unwrap();
    let times = lines.pop().unwrap();
    let result: u32 = times
        .zip(distances)
        .map(|(time, distance)| {
            (1..time)
                .map(move |x| (time - x) * x)
                .filter(move |&x| x > distance)
                .count() as u32
        })
        .product();
    println!("{result:?}");
}
