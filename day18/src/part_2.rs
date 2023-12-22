use crate::{Direction, Step, Tile};

pub(crate) fn part_2(input: &str) -> usize {
    let steps = parse_input(input);

    let positions = localize_steps(&steps);

    let lava_blocks = map_positions(&positions);

    println!("{lava_blocks}");
    lava_blocks
}

fn parse_input(input: &str) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let (_, color) = line.split_once('#').unwrap();
        let mut color_cleaned = color.replace(')', "");
        let direction = match color_cleaned.pop().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            c => unreachable!("Unexpected character {c}"),
        };
        let number = <usize>::from_str_radix(&color_cleaned, 16).unwrap();
        steps.push(Step::new(direction, number));
    }
    steps
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn scale(&mut self, x_min: usize, y_min: usize) {
        self.x -= x_min;
        self.y -= y_min;
    }
}

fn localize_steps(steps: &[Step]) -> Vec<(Step, Coord)> {
    let mut positions: Vec<(Step, Coord)> = Vec::new();
    let mut position = (
        Step {
            direction: Direction::Up,
            number: 0,
        },
        Coord::new(usize::MAX / 2, usize::MAX / 2),
    );

    positions.push(position);

    for step in steps {
        position.0 = *step;
        positions.push(position);
        match step.direction {
            Direction::Up => position.1.y -= step.number,
            Direction::Down => position.1.y += step.number,
            Direction::Left => position.1.x -= step.number,
            Direction::Right => position.1.x += step.number,
        }
    }

    let x_min = positions.iter().map(|(_, coord)| coord.x).min().unwrap();
    let y_min = positions.iter().map(|(_, coord)| coord.y).min().unwrap();

    for position in &mut positions {
        position.1.scale(x_min, y_min);
    }

    positions.remove(0);

    positions
}

fn map_positions(positions: &[(Step, Coord)]) -> usize {
    let n_cols = positions.iter().map(|(_, coord)| coord.x).max().unwrap();

    let mut total = 0;

    let mut important_rows: Vec<usize> = positions
        .iter()
        .filter(|(step, _)| {
            (step.direction == Direction::Left) | (step.direction == Direction::Right)
        })
        .map(|(_, coord)| coord.y)
        .collect();

    important_rows.sort_unstable();
    important_rows.dedup();

    for row in &important_rows {
        let map = build_row(positions.to_vec(), *row, n_cols);

        total += count_within(&map, n_cols + 1);
    }

    for i in 0..(important_rows.len() - 1) {
        let row = important_rows[i] + 1;
        let repetition = important_rows[i + 1] - important_rows[i] - 1;

        let map = build_row(positions.to_vec(), row, n_cols);

        total += repetition * count_within(&map, n_cols + 1);
    }

    total
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

fn build_row(positions: Vec<(Step, Coord)>, row: usize, n_cols: usize) -> Vec<Tile> {
    let mut map = vec![Tile::XX; n_cols + 1];

    let mut last_position = *positions.last().unwrap();

    for position in positions {
        let (step, coord) = position;
        let mut index = coord.x;

        match step.direction {
            Direction::Up => {
                if (coord.y == row) & (map[index] == Tile::XX) {
                    match last_position.0.direction {
                        Direction::Left => map[index] = Tile::UR,
                        Direction::Right => map[index] = Tile::UL,
                        _ => unreachable!(),
                    }
                }

                if (coord.y > row) & (coord.y - step.number < row) {
                    map[index] = Tile::UD;
                }
            }
            Direction::Down => {
                if (coord.y == row) & (map[index] == Tile::XX) {
                    match last_position.0.direction {
                        Direction::Left => map[index] = Tile::DR,
                        Direction::Right => map[index] = Tile::DL,
                        _ => unreachable!(),
                    }
                }

                if (coord.y < row) & (coord.y + step.number > row) {
                    map[index] = Tile::UD;
                }
            }
            Direction::Left => {
                if coord.y == row {
                    if map[index] == Tile::XX {
                        match last_position.0.direction {
                            Direction::Up => map[index] = Tile::DL,
                            Direction::Down => map[index] = Tile::UL,
                            _ => unreachable!(),
                        }
                    }
                    index -= 1;

                    for _ in 1..step.number {
                        map[index] = Tile::LR;
                        index -= 1;
                    }
                }
            }
            Direction::Right => {
                if coord.y == row {
                    if map[index] == Tile::XX {
                        match last_position.0.direction {
                            Direction::Up => map[index] = Tile::DR,
                            Direction::Down => map[index] = Tile::UR,
                            _ => unreachable!(),
                        }
                    }
                    index += 1;

                    for _ in 1..step.number {
                        map[index] = Tile::LR;
                        index += 1;
                    }
                }
            }
        }

        last_position = position;
    }

    map
}
