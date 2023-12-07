use roots::{Roots, find_roots_quadratic};

fn main() {
    let numbers: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars().filter(char::is_ascii_digit).collect::<String>().parse::<f64>().unwrap()
        })
        .collect();

    let [time, distance] = numbers[..2] else { unreachable!() };
    // -x² + time - distance = 0
    let ways = match find_roots_quadratic(-1f64, time, -distance) {
        Roots::Two([first, second]) => { second.floor() - first.ceil()},
        _ => unreachable!() 
    } as u32 + 1;
    
    println!("{ways}");
}
