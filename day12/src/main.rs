use itertools::Itertools;

fn main() {
    /*part_1(
        "#.#.### 1,1,3
        .#...#....###. 1,1,3
        .#.###.#.###### 1,3,1,6
        ####.#...#... 4,1,1
        #....######..#####. 1,6,5
        .###.##....# 3,2,1"
    )*/

    part_1(
        "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1",
    );

    part_2(
        "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1",
    );
}

fn part_1(input: &str) -> u32 {
    let mut total: u32 = 0;

    for line in input.lines() {
        let (states, groups) = parse_line(line);
        total += count_arrangements(&states, &groups);
    }

    println!("{total}");
    total
}

fn part_2(input: &str) -> u32 {
    let mut total: u32 = 0;

    for line in input.lines() {
        let (states, groups) = parse_line(line);
        let states_unfolded = states.repeat(3);
        let groups_unfolded = groups.repeat(3);
        total += count_arrangements(&states_unfolded, &groups_unfolded);
    }

    println!("{total}");
    total
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

fn parse_line(line: &str) -> (Vec<State>, Vec<usize>) {
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

fn group_states(states: &[State]) -> Vec<usize> {
    let mut groups: Vec<usize> = Vec::new();

    let mut count: usize = 0;

    for state in states {
        if state == &State::Damaged {
            count += 1;
        } else if count > 0 {
            groups.push(count);
            count = 0;
        }
    }

    if states[states.len() - 1] == State::Damaged {
        groups.push(count);
    }

    groups
}

fn check_states(states: &[State], groups: &[usize]) -> bool {
    let grouped_states = group_states(states);

    check_groups(&grouped_states, groups)
}

fn check_groups(groups_1: &[usize], groups_2: &[usize]) -> bool {
    if groups_1.len() != groups_2.len() {
        return false;
    }

    for i in 0..groups_1.len() {
        if groups_1[i] != groups_2[i] {
            return false;
        }
    }

    true
}

fn count_arrangements(states: &[State], groups: &[usize]) -> u32 {
    let total_damaged: usize = groups.iter().sum();

    let known_damaged = states
        .iter()
        .filter(|state| state == &&State::Damaged)
        .count();

    let missing_damaged = total_damaged - known_damaged;

    let mut unknown_positions = Vec::new();

    let mut count: u32 = 0;

    for (i, state) in states.iter().enumerate() {
        if state == &State::Unknown {
            unknown_positions.push(i);
        }
    }

    let mut new_states = states.to_owned();

    for combination in unknown_positions
        .clone()
        .into_iter()
        .combinations(missing_damaged)
    {
        for i in &combination {
            new_states[*i] = State::Damaged;
        }

        for i in &unknown_positions {
            if !combination.contains(i) {
                new_states[*i] = State::Operational;
            }
        }

        if check_states(&new_states, groups) {
            count += 1;
        }
    }

    count
}
