#![feature(slice_group_by)]
use array2d::Array2D;
use std::{cmp::{max, min}, ops::RangeBounds};

fn main() {
    let instructions: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let [dir, value] = line.split_whitespace().collect::<Vec<&str>>()[..2]
                .try_into()
                .unwrap();
            let value: isize = value.parse().unwrap();
            (dir, value)
        })
        .collect();

    //for (dir, value) in instructions {
    //    println!("{dir} {value}");
    //}

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut x = 0;
    let mut y = 0;

    for (dir, value) in &instructions {
        match *dir {
            "R" => {
                x += value;
                max_x = max(max_x, x);
            }
            "L" => {
                x -= value;
                min_x = min(min_x, x);
            }
            "D" => {
                y += value;
                max_y = max(max_y, y);
            }
            "U" => {
                y -= value;
                min_y = min(min_y, y);
            }
            _ => unreachable!(),
        }
    }

    let width = 1 + max_x - min_x;
    let height = 1 + max_y - min_y;

    let mut plan = Array2D::filled_with('.', height as usize, width as usize);
    let mut x = -min_x as usize;
    let mut y = -min_y as usize;
    plan.set(y, x, '#').unwrap();

    for (dir, value) in &instructions {
        let value = *value as usize;
        match *dir {
            "R" => {
                for i in x + 1..=x + value {
                    plan.set(y, i, '#').unwrap();
                }
                x += value;
            }
            "L" => {
                for i in x - value..x {
                    plan.set(y, i, '#').unwrap();
                }
                x -= value;
            }
            "D" => {
                for i in y + 1..=y + value {
                    plan.set(i, x, '#').unwrap();
                }
                y += value;
            }
            "U" => {
                for i in y - value..y {
                    plan.set(i, x, '#').unwrap();
                }
                y -= value;
            }
            _ => unreachable!(),
        }
    }

    //for y in 0..plan.column_len() {
    //    print!("{y} ");
    //    for x in 0..plan.row_len() {
    //        print!("{}", plan.get(y, x).unwrap());
    //    }
    //    println!();
    //}

    let plan = plan.as_rows();

    let mut count = 0;
    for (y, row) in plan.iter().enumerate() {
        print!("{y:6} {count:6} ");
        for c in row {
            print!("{c}");
        }
        let mut in_groups = Vec::new();
        for group in row.group_by(|a, b| a == b) {
            if group[0] == '#' {
                count += group.len();
            } else {
                in_groups.push(group);
            }
        }

        if in_groups.len() > 2 {
            let mut count_ins = if row[0] == '#' {1} else {0};
            for in_group in in_groups {
                if count_ins % 2 != 0 {
                    count += in_group.len();
                }
                count_ins += 1;
            }
        }

        println!();
    }

    println!("{count}");
}
