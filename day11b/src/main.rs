fn main() {
    let mut image: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|c| c.chars().collect())
        .collect();

    for y in 0..image.len() {
        if image[y].iter().all(|&c| c == '.') {
            image[y].fill('*');
        }
    }

    for x in 0..image[0].len() {
        if (0..image.len())
            .into_iter()
            .all(|y| image[y][x] == '.' || image[y][x] == '*')
        {
            for y in 0..image.len() {
                image[y][x] = '*';
            }
        }
    }

    let mut galaxies = Vec::new();
    for y in 0..image.len() {
        for x in 0..image[y].len() {
            if image[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
    }

    let mut count = 0;
    const EXPANSION: i64 = 1000000;
    for (i, (x1, y1)) in galaxies[..galaxies.len() - 1].iter().enumerate() {
        for (x2, y2) in &galaxies[i + 1..] {
            let x_range = if x1 < x2 {
                *x1 + 1..=*x2
            } else {
                *x2 + 1..=*x1
            };
            for x in x_range {
                if image[*y1][x] == '*' {
                    count += EXPANSION;
                } else {
                    count += 1;
                }
            }

            let y_range = if y1 < y2 {
                *y1 + 1..=*y2
            } else {
                *y2 + 1..=*y1
            };
            for y in y_range {
                if image[y][*x2] == '*' {
                    count += EXPANSION;
                } else {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");

}
