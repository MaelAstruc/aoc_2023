use std::fmt::Display;


pub(crate) fn part_2(input: &str) -> u32 {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().len();

    let mut map = parse_input(input);

    let mut positions: Vec<usize> = vec![n_cols, 1];

    map[1].cumulated_left[0] = Some(map[1].value);
    map[n_cols].cumulated_up[0] = Some(map[n_cols].value);

    while let Some(position) = positions.pop() {
        let current_tile = map[position];

        if position > n_cols {
            let index = position - n_cols;
            let previous = map[index];

            map[index].update(Direction::Down, &current_tile);

            if map[index] != previous {
                positions.push(index);
            }
        }
        if position < (n_rows - 1) * n_cols {
            let index = position + n_cols;
            let previous = map[index];

            map[index].update(Direction::Up, &current_tile);

            if map[index] != previous {
                positions.push(index);
            }
        }
        if position % n_cols != 0 {
            let index = position - 1;
            let previous = map[index];

            map[index].update(Direction::Right, &current_tile);

            if map[index] != previous {
                positions.push(index);
            }
        }
        if position % n_cols != n_cols - 1 {
            let index = position + 1;
            let previous = map[index];

            map[index].update(Direction::Left, &current_tile);

            if map[index] != previous {
                positions.push(index);
            }
        }

        //println!("{position}: {}", map[position]);
    }

    let last_tile = map[n_rows * n_cols - 1];

    let min_up = min_vector(&last_tile.cumulated_up[3..]);
    let min_down = min_vector(&last_tile.cumulated_down[3..]);
    let min_left = min_vector(&last_tile.cumulated_left[3..]);
    let min_right = min_vector(&last_tile.cumulated_right[3..]);

    let heat_loss = min_vector(&[min_up, min_down, min_left, min_right]).unwrap();

    println!("{heat_loss}");
    heat_loss
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Tile {
    value: u32,
    cumulated_up: [Option<u32>; 10],
    cumulated_down: [Option<u32>; 10],
    cumulated_left: [Option<u32>; 10],
    cumulated_right: [Option<u32>; 10],
}

impl Tile {
    fn new(value: u32) -> Self {
        Self {
            value,
            cumulated_up: [None; 10],
            cumulated_down: [None; 10],
            cumulated_left: [None; 10],
            cumulated_right: [None; 10],
        }
    }

    fn update(&mut self, from: Direction, tile: &Tile) {
        let min_up = min_vector(&tile.cumulated_up[3..]);
        let min_down = min_vector(&tile.cumulated_down[3..]);
        let min_left = min_vector(&tile.cumulated_left[3..]);
        let min_right = min_vector(&tile.cumulated_right[3..]);

        match from {
            Direction::Up => {
                self.cumulated_up[0] = min_vector(&[min_left, min_right]).map(|x| x + self.value);
                for i in 1..10 {
                    self.cumulated_up[i] = min_vector(&[
                        self.cumulated_up[i],
                        tile.cumulated_up[i-1].map(|x| x + self.value),
                    ]);
                }
            }
            Direction::Down => {
                self.cumulated_down[0] = min_vector(&[min_left, min_right]).map(|x| x + self.value);
                for i in 1..10 {
                    self.cumulated_down[i] = min_vector(&[
                        self.cumulated_down[i],
                        tile.cumulated_down[i-1].map(|x| x + self.value),
                    ]);
                }
            }
            Direction::Left => {
                self.cumulated_left[0] = min_vector(&[min_up, min_down]).map(|x| x + self.value);
                for i in 1..10 {
                    self.cumulated_left[i] = min_vector(&[
                        self.cumulated_left[i],
                        tile.cumulated_left[i-1].map(|x| x + self.value),
                    ]);
                }
            }
            Direction::Right => {
                self.cumulated_right[0] = min_vector(&[min_up, min_down]).map(|x| x + self.value);
                for i in 1..10 {
                    self.cumulated_right[i] = min_vector(&[
                        self.cumulated_right[i],
                        tile.cumulated_right[i-1].map(|x| x + self.value),
                    ]);
                }
            }
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\tvalue: {:?},\n\tUp: {:?},\n\tDown: {:?},\n\tLeft: {:?},\n\tRight: {:?}\n}}",
            self.value,
            self.cumulated_up,
            self.cumulated_down,
            self.cumulated_left,
            self.cumulated_right
        )
    }
}

fn min_vector(vector: &[Option<u32>]) -> Option<u32> {
    vector
        .iter()
        .flatten()
        .min()
        .copied()
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| Tile::new(c.to_digit(10).unwrap()))
        .collect()
}
