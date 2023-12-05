fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let mut seeds: Vec<_> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

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

    for seed in &mut seeds {
        for map in &maps {
            for (source, destination) in map {
                if source.contains(&seed){
                    *seed = destination.start + (*seed - source.start);
                    break;
                }
            }
        }
    }

    println!("{}", seeds.iter().min().unwrap());
}
