use std::collections::HashMap;

pub(crate) fn part_2(input: &str, steps: usize) {
    let (map, start, n_row, n_col) = parse_input(input);

    let mut distances: HashMap<Point, usize> = HashMap::new();
    distances.insert(start, 0);

    let mut positions: Vec<Point> = Vec::new();
    positions.push(start);

    let mut total: usize = 0;

    for step in 0..steps {
        let mut new_positions: Vec<Point> = Vec::new();
        while let Some(position) = positions.pop() {
            let distance = *distances.get(&position).unwrap();

            let point_up = position.new_up();

            let index = point_up.to_index(n_row, n_col);
            match (map[index], distances.get(&point_up)) {
                (Tile::Empty, None) => {
                    distances.insert(point_up, distance + 1);
                    new_positions.push(point_up);
                }
                (Tile::Empty, Some(_)) => (),
                (Tile::Rock, _) => (),
            };

            let point_down = position.new_down();

            let index = point_down.to_index(n_row, n_col);
            match (map[index], distances.get(&point_down)) {
                (Tile::Empty, None) => {
                    distances.insert(point_down, distance + 1);
                    new_positions.push(point_down);
                }
                (Tile::Empty, Some(_)) => (),
                (Tile::Rock, _) => (),
            };

            let point_left = position.new_left();

            let index = point_left.to_index(n_row, n_col);

            match (map[index], distances.get(&point_left)) {
                (Tile::Empty, None) => {
                    distances.insert(point_left, distance + 1);
                    new_positions.push(point_left);
                }
                (Tile::Empty, Some(_)) => (),
                (Tile::Rock, _) => (),
            };

            let point_right = position.new_right();

            let index = point_right.to_index(n_row, n_col);
            match (map[index], distances.get(&point_right)) {
                (Tile::Empty, None) => {
                    distances.insert(point_right, distance + 1);
                    new_positions.push(point_right);
                }
                (Tile::Empty, Some(_)) => (),
                (Tile::Rock, _) => (),
            };
        }

        new_positions.sort();
        new_positions.dedup();
        positions = new_positions;

        if step % 2 == 1 {
            let mut new_total: usize = 0;

            for distance in distances.values() {
                if distance % 2 == 1 {
                    new_total += 1;
                }
            }

            println!("{new_total}");

            total = new_total;
        }
    }

    println!("{total}");
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Rock,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new_up(&self) -> Self {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn new_down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn new_left(&self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn new_right(&self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn from_index(index: usize, n_row: usize, n_col: usize) -> Self {
        Self {
            row: (index / n_row) as isize,
            col: (index % n_col) as isize,
        }
    }

    fn to_index(&self, n_row: usize, n_col: usize) -> usize {
        let mut row_clean = self.row % n_row as isize;
        if row_clean < 0 {
            row_clean += n_row as isize;
        };
        let mut col_clean = self.col % n_col as isize;
        if col_clean < 0 {
            col_clean += n_col as isize;
        };
        (row_clean * n_row as isize + col_clean) as usize
    }
}

fn parse_input(input: &str) -> (Vec<Tile>, Point, usize, usize) {
    let mut map: Vec<Tile> = Vec::new();
    let mut start: Option<Point> = None;

    let n_row = input.lines().count();
    let n_col = input.lines().next().unwrap().len();

    for line in input.lines() {
        for c in line.trim().chars() {
            match c {
                '.' => map.push(Tile::Empty),
                '#' => map.push(Tile::Rock),
                'S' => {
                    start = Some(Point::from_index(map.len(), n_row, n_col));
                    map.push(Tile::Empty);
                }
                _ => unreachable!("Unexpected character {c}"),
            }
        }
    }

    (map, start.unwrap(), n_row, n_col)
}
