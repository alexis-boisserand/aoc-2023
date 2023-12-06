#![feature(iter_array_chunks)]

use core::ops::Range;

fn map_range(range: Range<i64>, map: &[(Range<i64>, Range<i64>)]) -> Vec<Range<i64>> {
    let mut ranges = Vec::new();
    for (i, (src, dst)) in map.iter().enumerate() {
        if src.end < range.start || range.end < src.start {
            continue;
        }

        if src.start <= range.start && src.end >= range.end {
            let dist = dst.start - src.start;
            ranges.push(range.start + dist..range.end + dist);
            return ranges;
        }

        if src.start <= range.start && src.end < range.end {
            let dist = dst.start - src.start;
            ranges.push(range.start + dist..src.end + dist);
            ranges.extend(map_range(src.end..range.end, &map[i + 1..]));
            return ranges;
        }

        if src.start > range.start && src.end >= range.end {
            ranges.push(dst.start..dst.start + range.end - src.start);
            ranges.extend(map_range(range.start..src.start, &map[i + 1..]));
            return ranges;
        }

        if src.start > range.start && src.end < range.end {
            ranges.push(dst.start..dst.end);
            ranges.extend(map_range(range.start..src.start, &map[i + 1..]));
            ranges.extend(map_range(src.end..range.end, &map[i + 1..]));
            return ranges;
        }
    }
    ranges.push(range);
    ranges
}

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
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect();

                    let (dst, src, len) = (numbers[0], numbers[1], numbers[2]);
                    ((src..src + len), (dst..dst + len))
                })
                .collect::<Vec<_>>()
        })
        .collect();


    let mut ranges: Vec<_> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .array_chunks()
        .map(|[start, length]| start..start + length)
        //.inspect(|x| print!("{x} "))
        .into_iter()
        .collect();

    for map in &maps {
        ranges = ranges
            .into_iter()
            .map(|range| map_range(range, map))
            .flatten()
            .collect();
    }

    let min = ranges.iter().map(|r| r.start).min().unwrap();
    println!("{min}");
}
