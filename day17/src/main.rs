mod part_1;
mod part_2;

use crate::part_1::part_1;
use crate::part_2::part_2;

fn main() {
    assert_eq!(
        part_1(
            "2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533"
        ),
        102
    );
    assert_eq!(
        part_2(
            "111111111111
            999999999991
            999999999991
            999999999991
            999999999991"
        ),
        71
    );

    assert_eq!(
        part_2(
            "2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533"
        ),
        94
    );
}