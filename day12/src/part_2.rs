use std::usize;

use memoise::memoise;

use crate::parse_line;
use crate::State;

pub(crate) fn part_2(input: &str) -> u64 {
    let mut total: u64 = 0;

    for (line_id, line) in input.lines().enumerate() {
        let (mut states, mut groups) = parse_line(line);

        states.push(State::Unknown);
        states = states.repeat(5);
        states.pop();
        groups = groups.repeat(5);

        let operational_known: usize = states
            .iter()
            .filter(|state| state == &&State::Operational)
            .count();
        let operational_total: usize = states.len() - groups.iter().sum::<usize>();

        let count = count_arrangements(
            line_id,
            &states,
            &groups,
            0,
            0,
            operational_known,
            operational_total,
        );

        total += count;
    }

    println!("{total}");
    total
}

#[memoise(line_id, states_id, group_id, operational_known, operational_total)]
fn count_arrangements(
    line_id: usize,
    states: &[State],
    groups: &[usize],
    states_id: usize,
    group_id: usize,
    operational_known: usize,
    operational_total: usize,
) -> u64 {
    // Check if state is empty
    match (states_id == states.len(), group_id == groups.len()) {
        (true, true) => return 1,
        (true, false) => return 0,
        (false, true) => {
            // If the states still include damaged we have an issue
            if states[states_id..]
                .iter()
                .filter(|state| state == &&State::Damaged)
                .count()
                > 0
            {
                return 0;
            }
            return 1;
        }
        (false, false) => (),
    }

    // If we have less states than damaged parts and intervals it's not possible
    if (states.len() - states_id + 1)
        < (groups[group_id..].iter().sum::<usize>() + groups.len() - group_id)
    {
        return 0;
    }

    // If there are too many operational we stop
    if operational_known > operational_total {
        return 0;
    }

    match states[states_id] {
        State::Operational => count_arrangements(
            line_id,
            states,
            groups,
            states_id + 1,
            group_id,
            operational_known,
            operational_total,
        ),
        State::Damaged => count_starts_damaged(
            line_id,
            states,
            groups,
            states_id,
            group_id,
            operational_known,
            operational_total,
        ),
        State::Unknown => {
            // The count if we assume it's an operational
            let operational_counts = count_arrangements(
                line_id,
                states,
                groups,
                states_id + 1,
                group_id,
                operational_known + 1,
                operational_total,
            );
            // The count if we assume it's a damaged
            let damaged_counts = count_starts_damaged(
                line_id,
                states,
                groups,
                states_id,
                group_id,
                operational_known,
                operational_total,
            );
            operational_counts + damaged_counts
        }
    }
}

#[memoise(line_id, states_id, group_id, operational_known, operational_total)]
fn count_starts_damaged(
    line_id: usize,
    states: &[State],
    groups: &[usize],
    states_id: usize,
    group_id: usize,
    operational_known: usize,
    operational_total: usize,
) -> u64 {
    // Check if state and group are empty
    match (states_id == states.len(), group_id == groups.len()) {
        (true, true) => return 1,
        (true, false) => return 0,
        (false, true) => {
            // If the states still include damaged we have an issue
            if states
                .iter()
                .filter(|state| state == &&State::Damaged)
                .count()
                > 0
            {
                return 0;
            }
            return 1;
        }
        (false, false) => (),
    }

    // If we have less states than damaged parts and intervals it's not possible
    if (states.len() - states_id + 1)
        < (groups[group_id..].iter().sum::<usize>() + groups.len() - group_id)
    {
        return 0;
    }

    let size: usize = groups[group_id];

    // All the states in the slice have to be damaged or unknown
    for state in states[states_id..].iter().take(size) {
        match state {
            State::Operational => return 0,
            State::Damaged | State::Unknown => (),
        }
    }

    // If the group touches the end we have a valid arrangement
    if states_id + size == states.len() {
        return 1;
    }

    // If we still have only one space we check that next one can be an operational
    if states_id + size + 1 == states.len() {
        match states[states_id + size] {
            State::Operational | State::Unknown => return 1,
            State::Damaged => return 0,
        };
    }

    // If there are no other group we check that the remaining are not damaged
    if group_id == groups.len() {
        for state in &states[(states_id + size + 1)..] {
            if state == &State::Damaged {
                return 0;
            }
        }
        return 1;
    }

    // Finally we check that the count for the remaining ones
    match states[states_id + size] {
        State::Operational => count_arrangements(
            line_id,
            states,
            groups,
            states_id + size + 1,
            group_id + 1,
            operational_known,
            operational_total,
        ),
        State::Damaged => 0,
        State::Unknown => count_arrangements(
            line_id,
            states,
            groups,
            states_id + size + 1,
            group_id + 1,
            operational_known + 1,
            operational_total,
        ),
    }
}
