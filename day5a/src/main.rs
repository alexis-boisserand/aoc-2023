fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let maps: Vec<_> = lines[2..]
        .split(|line| line.is_empty())
        .map(|lines| {
            lines[1..]
                .iter()
                .map(|line| {
                    let numbers: Vec<_> = line
                        .split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect();

                    let (dst, src, len) = (numbers[0], numbers[1], numbers[2]);
                    ((src..src + len), (dst..dst + len))
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let min = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |value, map| {
                map.iter()
                    .find_map(|(src, dst)| {
                        src.contains(&value)
                            .then(|| dst.start + value - src.start)
                    })
                    .unwrap_or(value)
            })
        })
        .min()
        .unwrap();

    println!("{min}");
}
