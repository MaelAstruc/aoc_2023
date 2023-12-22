mod part_1;
mod part_2;

use crate::part_1::part_1;
use crate::part_2::part_2;

fn main() {
    assert_eq!(
        part_1(
            "R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)",
        ),
        62
    );

    assert_eq!(
        part_2(
            "R 6 (#60)
            D 5 (#51)
            L 2 (#22)
            D 2 (#21)
            R 2 (#20)
            D 2 (#21)
            L 5 (#52)
            U 2 (#23)
            L 1 (#12)
            U 2 (#23)
            R 2 (#20)
            U 3 (#33)
            L 2 (#22)
            U 2 (#23)"
        ),
        62
    );

    assert_eq!(
        part_2(
            "R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)",
        ),
        952_408_144_115
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    UD,
    LR,
    UR,
    UL,
    DL,
    DR,
    XX,
}

#[derive(Debug, Clone, Copy)]
struct Step {
    direction: Direction,
    number: usize,
}

impl Step {
    fn new(direction: Direction, number: usize) -> Self {
        Self { direction, number }
    }
}
