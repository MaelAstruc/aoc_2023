mod part_1;
mod part_2;

use crate::part_1::part_1;
use crate::part_2::part_2;

fn main() {
    part_1(
        "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........",
        6,
    );

    part_1(&std::fs::read_to_string("input.txt").unwrap(), 64);

    part_2(
        "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........",
        6,
    );

    part_2(&std::fs::read_to_string("input.txt").unwrap(), 1000);
}
