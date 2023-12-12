use Direction::*;
use Tile::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    South,
    North,
    East,
    West,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            South => North,
            North => South,
            East => West,
            West => East,
        }
    }
}

#[derive(Debug)]
enum Tile {
    Ground,
    Start,
    Pipe(Direction, Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe(North, South),
            '-' => Pipe(East, West),
            'L' => Pipe(North, East),
            'J' => Pipe(North, West),
            '7' => Pipe(South, West),
            'F' => Pipe(South, East),
            '.' => Ground,
            'S' => Start,
            _ => unreachable!(),
        }
    }
}

fn next(x: usize, y: usize, direction: Direction, map: &[Vec<Tile>]) -> Option<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    let (x, y) = match direction {
        South => (x, y + 1),
        North => (x, y - 1),
        East => (x + 1, y),
        West => (x - 1, y),
    };
    if x >= 0 && y >= 0 && y < map.len() as isize && x < map[0].len() as isize {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

fn main() {
    let map: Vec<Vec<Tile>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .position(|t| matches!(t, Tile::Start))
                .map(|x| (x, y))
        })
        .unwrap();

    let mut tile = None;
    let mut direction = None;
    for dir in [West, East, South, North] {
        if let Some((x, y)) = next(start_x, start_y, dir, &map) {
            if let Pipe(dir1, dir2) = &map[y][x] {
                let opposite = dir.opposite();
                if *dir1 == opposite || *dir2 == opposite {
                    tile = Some((x, y));
                    direction = Some(if *dir1 == opposite { *dir2 } else { *dir1 });
                    break;
                }
            }
        }
    }

    let (mut x, mut y) = tile.unwrap();
    let mut direction = direction.unwrap();

    let mut count = 1u32;
    loop {
        println!("{x} {y} {direction:?} {:?}", map[y][x]);
        (x, y) = next(x, y, direction, &map).unwrap();
        count += 1;
        match &map[y][x] {
            Pipe(dir1, dir2) => {
                let opposite = direction.opposite();
                direction = if *dir1 == opposite { *dir2 } else { *dir1 };
            }
            Start => {
                break;
            }
            _ => unreachable!(),
        }
    }

    let count = if count % 2 == 0 {
        count / 2
    } else {
        count / 2 + 1
    };

    println!("{count}");
}
