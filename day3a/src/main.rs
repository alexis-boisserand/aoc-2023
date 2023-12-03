fn is_symbol(c: char) -> bool {
    c != '.' && c.is_ascii_punctuation()
}

fn symbol_above(index: usize, lines: &[&str], mut start: usize, mut end: usize) -> bool {
    if index == 0 {
        false
    } else {
        start = if start == 0 { start } else { start - 1 };
        end = if end == lines[index].len() {
            end
        } else {
            end + 1
        };
        lines[index - 1]
            .chars()
            .skip(start)
            .take(end - start)
            .any(is_symbol)
    }
}

fn symbol_on_left(line: &str, start: usize) -> bool {
    if start == 0 {
        false
    } else {
        is_symbol(line.chars().nth(start - 1).unwrap())
    }
}

fn symbol_on_right(line: &str, end: usize) -> bool {
    if end == line.len() {
        false
    } else {
        is_symbol(line.chars().nth(end).unwrap())
    }
}

fn symbol_below(index: usize, lines: &[&str], mut start: usize, mut end: usize) -> bool {
    if index == lines.len() - 1 {
        false
    } else {
        start = if start == 0 { start } else { start - 1 };
        end = if end == lines[index].len() {
            end
        } else {
            end + 1
        };
        lines[index + 1]
            .chars()
            .skip(start)
            .take(end - start)
            .any(is_symbol)
    }
}

fn part_numbers_sum(line: &str, index: usize, lines: &[&str]) -> u32 {
    let mut iter = line.chars().enumerate();
    let mut indices: Vec<(usize, usize)> = Vec::new();
    while let Some((i, ch)) = iter.next() {
        if ch.is_digit(10) {
            let mut found = false;
            while let Some((j, ch)) = iter.next() {
                if !ch.is_digit(10) {
                    indices.push((i, j));
                    found = true;
                    break;
                }
            }
            if !found {
                indices.push((i, line.len()));
            }
        }
    }
    let mut numbers: Vec<u32> = Vec::new();
    for (start, end) in indices {
        if symbol_above(index, lines, start, end)
            || symbol_on_left(line, start)
            || symbol_on_right(line, end)
            || symbol_below(index, lines, start, end)
        {
            numbers.push(line[start..end].parse().unwrap());
        }
    }
    println!("{line} {numbers:?}");
    numbers.iter().sum()
}

fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let sum: u32 = lines
        .iter()
        .enumerate()
        .map(|(index, line)| part_numbers_sum(line, index, &lines))
        .sum();
    println!("{sum}");
}
