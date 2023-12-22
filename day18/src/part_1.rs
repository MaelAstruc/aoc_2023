use crate::{Direction, Step, Tile};

pub(crate) fn part_1(input: &str) -> usize {
    let steps = parse_input(input);

    let positions = localize_steps(&steps);

    let (map, line_size) = map_positions(&positions);

    let lava_blocks = count_within(&map, line_size);

    println!("{lava_blocks}");
    lava_blocks
}

fn parse_input(input: &str) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let mut splitted = line.split(' ');
        let direction = match splitted.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            s => unreachable!("Unexpected string {s}"),
        };
        let number = splitted.next().unwrap().parse::<usize>().unwrap();
        steps.push(Step::new(direction, number));
    }
    steps
}

fn map_positions(positions: &[(Direction, usize, usize)]) -> (Vec<Tile>, usize) {
    let n_cols = positions.iter().map(|(_, x, _)| x).max().unwrap() + 1;
    let n_rows = positions.iter().map(|(_, _, y)| y).max().unwrap() + 1;
    let size = n_rows * n_cols;

    let mut map = vec![Tile::XX; size];

    let position = positions.last().unwrap();
    let mut last_position = position;
    let mut index = position.2 * n_cols + position.1;
    let first_index = index;

    for position in positions {
        map[index] = match (last_position.0, position.0) {
            (Direction::Up, Direction::Left) => Tile::DL,
            (Direction::Up, Direction::Right) => Tile::DR,
            (Direction::Down, Direction::Left) => Tile::UL,
            (Direction::Down, Direction::Right) => Tile::UR,
            (Direction::Left, Direction::Up) => Tile::UR,
            (Direction::Left, Direction::Down) => Tile::DR,
            (Direction::Right, Direction::Up) => Tile::UL,
            (Direction::Right, Direction::Down) => Tile::DL,
            (last, current) => unreachable!("Unexpected case {last:?}, {current:?}"),
        };

        match position.0 {
            Direction::Up => {
                let y_diff = last_position.2 - position.2;
                for _ in 1..=y_diff {
                    index -= n_cols;
                    if index != first_index {
                        map[index] = Tile::UD;
                    }
                }
            }
            Direction::Down => {
                let y_diff = position.2 - last_position.2;
                for _ in 1..=y_diff {
                    index += n_cols;
                    if index != first_index {
                        map[index] = Tile::UD;
                    }
                }
            }
            Direction::Left => {
                let x_diff = last_position.1 - position.1;
                for _ in 1..=x_diff {
                    index -= 1;
                    if index != first_index {
                        map[index] = Tile::LR;
                    }
                }
            }
            Direction::Right => {
                let x_diff = position.1 - last_position.1;
                for _ in 1..=x_diff {
                    index += 1;
                    if index != first_index {
                        map[index] = Tile::LR;
                    }
                }
            }
        }
        last_position = position;
    }

    (map, n_cols)
}

fn localize_steps(steps: &[Step]) -> Vec<(Direction, usize, usize)> {
    let mut positions: Vec<(Direction, usize, usize)> = Vec::new();
    let mut position = (Direction::Up, usize::MAX / 2, usize::MAX / 2);

    positions.push(position);

    for step in steps {
        position.0 = step.direction;
        match step.direction {
            Direction::Up => position.2 -= step.number,
            Direction::Down => position.2 += step.number,
            Direction::Left => position.1 -= step.number,
            Direction::Right => position.1 += step.number,
        }
        positions.push(position);
    }

    let x_min = positions.iter().map(|(_, x, _)| x).min().unwrap();
    let y_min = positions.iter().map(|(_, _, y)| y).min().unwrap();
    positions = positions
        .iter()
        .map(|(direction, x, y)| (*direction, x - x_min, y - y_min))
        .collect();

    positions.remove(0);

    positions
}

// From day 10
fn count_within(map: &[Tile], line_size: usize) -> usize {
    let mut inside = true;
    let mut total: usize = 0;
    let mut last: Option<&Tile> = None;

    for (i, tile) in map.iter().enumerate() {
        if i % line_size == 0 {
            inside = false;
            last = None;
        }

        match (tile, last) {
            (Tile::XX, _) => {
                if inside {
                    total += 1;
                }
            }
            (Tile::UD, _) => {
                total += 1;
                inside = !inside;
            }
            (Tile::LR, _) => total += 1,
            (Tile::UR | Tile::DR, _) => {
                total += 1;
                inside = !inside;
                last = Some(tile);
            }
            (Tile::UL, Some(Tile::UR)) => {
                total += 1;
                inside = !inside;
            }
            (Tile::UL, Some(Tile::DR)) => total += 1,
            (Tile::DL, Some(Tile::UR)) => total += 1,
            (Tile::DL, Some(Tile::DR)) => {
                total += 1;
                inside = !inside;
            }
            (_, _) => unreachable!("{tile:?} should not follow {last:?} at {i}"),
        }
    }

    total
}
