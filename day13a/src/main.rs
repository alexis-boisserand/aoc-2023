use core::cmp::min;

use array2d::Array2D;

fn main() {
    let lines: Vec<_> = include_str!("input.txt").lines().collect();
    let patterns: Vec<_> = lines
        .split(|line| line.is_empty())
        .map(|lines| {
            Array2D::from_rows(
                &lines
                    .into_iter()
                    .map(|line| line.chars().collect::<Vec<char>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        })
        .collect();

    let mut count = 0;
    for pattern in &patterns {
        let columns = pattern.as_columns();
        let mut max_count_similar_columns = (0, 0);
        for x in 1..columns.len() - 1 {
            let range = min(x, columns.len() - x);
            let mut count_similar_columns = 0;
            for (left, right) in (x - range..x).rev().zip(x..x + range) {
                if columns[left] == columns[right] {
                    count_similar_columns += 1;
                } else {
                    break;
                }
            }
            if count_similar_columns > max_count_similar_columns.0 {
                // doesn't seem necessary to count here
                max_count_similar_columns = (count_similar_columns, x);
            }
        }

        let rows = pattern.as_rows();
        let mut max_count_similar_rows = (0, 0);
        for y in 1..rows.len() - 1 {
            let range = min(y, rows.len() - y);
            let mut count_similar_rows = 0;
            for (left, right) in (y - range..y).rev().zip(y..y + range) {
                if rows[left] == rows[right] {
                    count_similar_rows += 1;
                } else {
                    break;
                }
            }
            if count_similar_rows > max_count_similar_rows.0 {
                // doesn't seem necessary to count here
                max_count_similar_rows = (count_similar_rows, y);
            }
        }

        let value = if max_count_similar_columns.0 >= max_count_similar_rows.0 {
            max_count_similar_columns.1
        } else {
            max_count_similar_rows.1 * 100
        };

        count += value;

        //println!("{pattern:?}");
        println!("{value}");
    }

    println!("{count}");
}
