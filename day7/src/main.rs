mod part_1;
mod part_2;

use crate::part_1::part_1;
use crate::part_2::part_2;

fn main() {
    assert_eq!(
        part_1(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        ),
        6440
    );

    assert_eq!(
        part_2(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        ),
        5905
    );
}
