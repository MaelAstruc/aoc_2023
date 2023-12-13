mod part_1;
mod part_2;

use crate::part_1::part_1;
use crate::part_2::part_2;

fn main() {
    assert_eq!(
        part_1(
            "#.#.### 1,1,3
            .#...#....###. 1,1,3
            .#.###.#.###### 1,3,1,6
            ####.#...#... 4,1,1
            #....######..#####. 1,6,5
            .###.##....# 3,2,1"
        ),
        6
    );

    assert_eq!(
        part_1(
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1"
        ),
        21
    );

    assert_eq!(
        part_2(
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1",
        ),
        525_152
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

pub(crate) fn parse_line(line: &str) -> (Vec<State>, Vec<usize>) {
    let (records, list) = line.trim().split_once(' ').unwrap();

    let states = parse_records(records);

    let groups: Vec<usize> = list
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    (states, groups)
}

fn parse_records(records: &str) -> Vec<State> {
    let mut states: Vec<State> = Vec::new();

    for c in records.chars() {
        match c {
            '.' => states.push(State::Operational),
            '#' => states.push(State::Damaged),
            '?' => states.push(State::Unknown),
            _ => unreachable!("Unexpected character: {c}"),
        }
    }

    states
}
