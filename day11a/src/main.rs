fn main() {
    let mut image: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|c| c.chars().collect())
        .collect();

    let mut y = 0;
    while y < image.len() {
        if image[y].iter().all(|&c| c == '.') {
            image.insert(y, vec!['.'; image[y].len()]);
            y += 2;
        } else {
            y += 1;
        }
    }

    let mut x = 0;
    while x < image[0].len() {
        if (0..image.len()).into_iter().all(|y| image[y][x] == '.') {
            for y in 0..image.len() {
                image[y].insert(x, '.');
            }
            x += 2;
        } else {
            x += 1;
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
    for (i, (x1, y1)) in galaxies[..galaxies.len() - 1].iter().enumerate() {
        for (x2, y2) in &galaxies[i + 1..] {
            count += x1.abs_diff(*x2) + y1.abs_diff(*y2);
        }
    }
    println!("{count}");

    //for y in 0..image.len() {
    //    for x in 0..image[y].len() {
    //        print!("{}", image[y][x]);
    //    }
    //    println!();
    //}
}
