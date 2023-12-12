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

#[derive(Debug, Clone)]
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
    source_direction: Option<Direction>,
    map: &[Vec<Tile>],
    pipes: &HashSet<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
    depth: usize,
) -> bool {
    visited.insert((x, y));

    let mut success = false;

    for direction in ALL_DIRECTIONS {
        if source_direction.map_or(true, |source| direction != source.opposite()) {
            match next(x, y, direction, map) {
                None => {
                    return true;
                }
                Some((x, y)) => {
                    if !visited.contains(&(x, y)) && !pipes.contains(&(x, y)) {
                        success |= escape(x, y, Some(direction), map, pipes, visited, depth + 1);
                    }
                }
            }
        }
    }
    success
}

fn expand_map(
    map: &[Vec<Tile>],
    pipes: &HashSet<(usize, usize)>,
) -> (Vec<Vec<Tile>>, HashSet<(usize, usize)>) {
    let height = map.len();
    let width = map[0].len();

    let mut expanded_pipes = HashSet::new();
    let mut expanded_map = vec![vec![Ground; width * 2]; height * 2];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            expanded_map[y * 2][x * 2] = map[y][x].clone();
            if pipes.contains(&(x, y)) {
                expanded_pipes.insert((x * 2, y * 2));
                match map[y][x] {
                    HorizontalPipe | BentPipe(_, East) => {
                        expanded_map[y * 2][x * 2 + 1] = HorizontalPipe;
                        expanded_pipes.insert((x * 2 + 1, y * 2));
                    }
                    _ => {}
                }

                match map[y][x] {
                    VerticalPipe | BentPipe(South, _) => {
                        expanded_map[y * 2 + 1][x * 2] = VerticalPipe;
                        expanded_pipes.insert((x * 2, y * 2 + 1));
                    }
                    _ => {}
                }
            }
        }
    }

    (expanded_map, expanded_pipes)
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

    let (expanded_map, expanded_pipes) = expand_map(&map, &pipes);
    let mut count = 0;

    for y in 0..expanded_map.len() {
        for x in 0..expanded_map[y].len() {
            print!("{}", expanded_map[y][x]);
        }
        println!();
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !pipes.contains(&(x, y)) {
                let mut visited = HashSet::new();
                if !escape(
                    x * 2,
                    y * 2,
                    None,
                    &expanded_map,
                    &expanded_pipes,
                    &mut visited,
                    0,
                ) {
                    println!("{x} {y} didn't escape");
                    count += 1;
                }
            }
        }
    }

    println!("{count}");

    //let mut visited: HashSet<(usize, usize)> = HashSet::new();
    //let depth = 0;
    //escape(2, 6, &ALL_DIRECTIONS, &map, &pipes, &mut visited, depth);
}
