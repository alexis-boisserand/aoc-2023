use std::{collections::HashSet, fmt::Display};

use Direction::*;
use HorizontalDirection::*;
use Tile::*;
use VerticalDirection::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VerticalDirection {
    South,
    North,
}

impl VerticalDirection {
    fn opposite(self) -> Self {
        match self {
            North => South,
            South => North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HorizontalDirection {
    East,
    West,
}

impl HorizontalDirection {
    fn opposite(self) -> Self {
        match self {
            West => East,
            East => West,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Vertical(VerticalDirection),
    Horizontal(HorizontalDirection),
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Vertical(South),
    Vertical(North),
    Horizontal(West),
    Horizontal(East),
];

const ALL_VERTICAL_DIRECTIONS: [Direction; 2] = [Vertical(South), Vertical(North)];

const ALL_HORIZONTAL_DIRECTIONS: [Direction; 2] = [Horizontal(West), Horizontal(East)];

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Vertical(d) => Vertical(d.opposite()),
            Horizontal(d) => Horizontal(d.opposite()),
        }
    }
}

#[derive(Debug)]
enum Tile {
    Ground,
    Start,
    HorizontalPipe,
    VerticalPipe,
    BentPipe(VerticalDirection, HorizontalDirection),
}

impl Tile {
    fn follow(&self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (VerticalPipe, Vertical(_)) => Some(direction),
            (HorizontalPipe, Horizontal(_)) => Some(direction),
            (BentPipe(v, h), Vertical(d)) => (v.opposite() == d).then_some(Horizontal(*h)),
            (BentPipe(v, h), Horizontal(d)) => (h.opposite() == d).then_some(Vertical(*v)),
            _ => None,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => VerticalPipe,
            '-' => HorizontalPipe,
            'L' => BentPipe(North, East),
            'J' => BentPipe(North, West),
            '7' => BentPipe(South, West),
            'F' => BentPipe(South, East),
            '.' => Ground,
            'S' => Start,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VerticalPipe => "|",
            HorizontalPipe => "-",
            BentPipe(North, East) => "L",
            BentPipe(North, West) => "J",
            BentPipe(South, West) => "7",
            BentPipe(South, East) => "F",
            Ground => ".",
            Start => "S",
        };
        f.write_str(s)
    }
}

fn next(x: usize, y: usize, direction: Direction, map: &[Vec<Tile>]) -> Option<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    let (x, y) = match direction {
        Vertical(South) => (x, y + 1),
        Vertical(North) => (x, y - 1),
        Horizontal(East) => (x + 1, y),
        Horizontal(West) => (x - 1, y),
    };
    if x >= 0 && y >= 0 && y < map.len() as isize && x < map[0].len() as isize {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

fn escape(
    x: usize,
    y: usize,
    directions: &[Direction],
    map: &[Vec<Tile>],
    pipes: &HashSet<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
    depth: usize,
) -> bool {
    let space = str::repeat(" ", depth);
    println!("{space}visiting {x} {y}");
    visited.insert((x, y));

    let mut success = false;

    for direction in directions {
        match next(x, y, *direction, map) {
            None => {
                return true;
            }
            Some((x, y)) => {
                if !visited.contains(&(x, y)) {
                    let mut next_directions: Vec<Direction> = Vec::new();
                    if !pipes.contains(&(x, y)) {
                        next_directions.extend(ALL_DIRECTIONS);
                    } else {
                        match (direction, &map[y][x]) {
                            (Horizontal(_), HorizontalPipe) => {
                                next_directions.extend(ALL_HORIZONTAL_DIRECTIONS);
                            }
                            (Vertical(_), VerticalPipe) => {
                                next_directions.extend(ALL_VERTICAL_DIRECTIONS);
                            }
                            (Vertical(_) | Horizontal(_), BentPipe(_, _)) => {
                                next_directions.push(*direction);
                            }
                            _ => {}
                        }
                    }

                    let next_directions: Vec<_> = next_directions
                        .into_iter()
                        .filter(|d| *d != direction.opposite())
                        .collect();
                    if !next_directions.is_empty() {
                        success |= escape(x, y, &next_directions, map, pipes, visited, depth + 1);
                    }
                }
                //    if !pipes.contains(&(x, y))
                //        || match (direction, &map[y][x]) {
                //            (Horizontal(_), HorizontalPipe) | (Vertical(_), VerticalPipe) => {
                //                true
                //            }
                //            (Vertical(_) | Horizontal(_), BentPipe(_, _)) => true,
                //            _ => false,
                //        }
                //    {
                //        success |=
                //            escape(x, y, Some(direction), map, pipes, visited, depth + 1);
                //    }
                //}
            }
        }
    }
    success
}

fn main() {
    let mut map: Vec<Vec<Tile>> = include_str!("input.txt")
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
    let mut start_direction = None;
    for dir in [
        Horizontal(West),
        Horizontal(East),
        Vertical(South),
        Vertical(North),
    ] {
        if let Some((x, y)) = next(start_x, start_y, dir, &map) {
            let pipe = &map[y][x];

            if let od @ Some(_) = pipe.follow(dir) {
                tile = Some((x, y));
                direction = od;
                start_direction = Some(dir);
                break;
            }
        }
    }

    let (mut x, mut y) = tile.unwrap();
    let mut direction = direction.unwrap();
    let stop_direction;

    let mut pipes = vec![(start_x, start_y), (x, y)];
    loop {
        (x, y) = next(x, y, direction, &map).unwrap();
        pipes.push((x, y));

        match map[y][x].follow(direction) {
            Some(d) => {
                direction = d;
            }
            None => {
                stop_direction = direction.opposite();
                break;
            }
        }
    }

    let start_tile = match (start_direction.unwrap(), stop_direction) {
        (Vertical(_), Vertical(_)) => VerticalPipe,
        (Horizontal(_), Horizontal(_)) => HorizontalPipe,
        (Vertical(v), Horizontal(h)) | (Horizontal(h), Vertical(v)) => BentPipe(v, h),
        _ => unreachable!(),
    };

    map[start_y][start_x] = start_tile;

    let pipes: HashSet<(usize, usize)> = pipes.into_iter().collect();

    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !pipes.contains(&(x, y)) {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                if escape(x, y, &ALL_DIRECTIONS, &map, &pipes, &mut visited, 0) {
                    println!("{x} {y} escaped!");
                } else {
                    count += 1;
                    println!("{x} {y} didn't escape :(");
                }
            }
        }
    }
    println!("{count}");

    //let mut visited: HashSet<(usize, usize)> = HashSet::new();
    //let depth = 0;
    //escape(2, 6, &ALL_DIRECTIONS, &map, &pipes, &mut visited, depth);
}
