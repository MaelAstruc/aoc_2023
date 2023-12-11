mod part_1;
mod part_2;

use crate::part_1::part_1;
use crate::part_2::part_2;

fn main() {
    assert_eq!(
        part_1(
            ".....
            .S-7.
            .|.|.
            .L-J.
            ....."
        ),
        4
    );

    assert_eq!(
        part_1(
            "..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ..."
        ),
        8
    );

    assert_eq!(
        part_2(
            "...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ..........."
        ),
        4
    );

    assert_eq!(
        part_2(
            ".F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ..."
        ),
        8
    );

    assert_eq!(
        part_2(
            "FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L"
        ),
        10
    );
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn next_position(&self, position: usize, line_size: usize) -> usize {
        match self {
            Direction::North => position - line_size,
            Direction::South => position + line_size,
            Direction::East => position + 1,
            Direction::West => position - 1,
        }
    }
}

#[derive(Debug)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    fn crawl(&self, from: &Direction) -> Direction {
        match (self, from) {
            (Tile::Start, _) => todo!(),
            (Tile::NS, Direction::North) => Direction::South,
            (Tile::NS, Direction::South) => Direction::North,
            (Tile::EW, Direction::East) => Direction::West,
            (Tile::EW, Direction::West) => Direction::East,
            (Tile::NE, Direction::North) => Direction::East,
            (Tile::NE, Direction::East) => Direction::North,
            (Tile::NW, Direction::North) => Direction::West,
            (Tile::NW, Direction::West) => Direction::North,
            (Tile::SW, Direction::South) => Direction::West,
            (Tile::SW, Direction::West) => Direction::South,
            (Tile::SE, Direction::South) => Direction::East,
            (Tile::SE, Direction::East) => Direction::South,
            (_, _) => unreachable!("{self:?} cannot come from {:?}", from),
        }
    }
}

impl From<(Direction, Direction)> for Tile {
    fn from(value: (Direction, Direction)) -> Self {
        match (value.0, value.1) {
            (Direction::North, Direction::South) => Tile::NS,
            (Direction::North, Direction::East) => Tile::NE,
            (Direction::North, Direction::West) => Tile::NW,
            (Direction::South, Direction::North) => Tile::NS,
            (Direction::South, Direction::East) => Tile::SE,
            (Direction::South, Direction::West) => Tile::SW,
            (Direction::East, Direction::North) => Tile::NE,
            (Direction::East, Direction::South) => Tile::SE,
            (Direction::East, Direction::West) => Tile::EW,
            (Direction::West, Direction::North) => Tile::NW,
            (Direction::West, Direction::South) => Tile::SW,
            (Direction::West, Direction::East) => Tile::EW,
            (_, _) => unreachable!("Cannot go to the incoming direction: {:?}", value.0),
        }
    }
}
