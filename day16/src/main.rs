fn main() {
    assert_eq!(
        part_1(
            r".|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....",
        ),
        46
    );

    assert_eq!(
        part_2(
            r".|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....",
        ),
        51
    );
}

fn part_1(input: &str) -> u32 {
    let map = parse_input(input);

    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().len();

    let total = light(map, n_rows, n_cols, (0, Direction::Left));

    println!("{total}");

    total
}

fn part_2(input: &str) -> u32 {
    let map = parse_input(input);

    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().len();

    let mut max = 0;

    for i in 0..n_cols {
        let total = light(map.clone(), n_rows, n_cols, (i, Direction::Up));
        if total > max {
            max = total;
        }
    }

    for i in 0..n_cols {
        let total = light(
            map.clone(),
            n_rows,
            n_cols,
            ((n_rows - 1) * n_cols + i, Direction::Down),
        );
        if total > max {
            max = total;
        }
    }

    for i in 0..n_rows {
        let total = light(map.clone(), n_rows, n_cols, (i * n_cols, Direction::Left));
        if total > max {
            max = total;
        }
    }

    for i in 0..n_rows {
        let total = light(
            map.clone(),
            n_rows,
            n_cols,
            ((i + 1) * n_cols - 1, Direction::Right),
        );
        if total > max {
            max = total;
        }
    }

    println!("{max}");

    max
}

struct Positions {
    list: Vec<(usize, Direction)>,
    n_rows: usize,
    n_cols: usize,
}

impl Positions {
    fn new(n_rows: usize, n_cols: usize, first: (usize, Direction)) -> Self {
        Self {
            list: vec![first],
            n_rows,
            n_cols,
        }
    }

    fn pop(&mut self) -> Option<(usize, Direction)> {
        self.list.pop()
    }

    fn push_up(&mut self, position: usize) {
        if position >= self.n_cols {
            self.list.push((position - self.n_cols, Direction::Down));
        }
    }

    fn push_down(&mut self, position: usize) {
        if position < (self.n_rows - 1) * self.n_cols {
            self.list.push((position + self.n_cols, Direction::Up));
        }
    }

    fn push_left(&mut self, position: usize) {
        if position % self.n_cols != 0 {
            self.list.push((position - 1, Direction::Right));
        }
    }

    fn push_right(&mut self, position: usize) {
        if position % self.n_cols != self.n_cols - 1 {
            self.list.push((position + 1, Direction::Left));
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitterVertical,
    SplitterHorizontal,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<(Tile, [bool; 4])> {
    let mut map: Vec<(Tile, [bool; 4])> = Vec::new();

    for c in input.chars() {
        match c {
            ' ' | '\t' | '\r' | '\n' => continue,
            '.' => map.push((Tile::Empty, [false; 4])),
            '\\' => map.push((Tile::MirrorDown, [false; 4])),
            '/' => map.push((Tile::MirrorUp, [false; 4])),
            '-' => map.push((Tile::SplitterHorizontal, [false; 4])),
            '|' => map.push((Tile::SplitterVertical, [false; 4])),
            _ => unreachable!("Unexpected character '{c}'"),
        }
    }

    map
}

fn light(
    mut map: Vec<(Tile, [bool; 4])>,
    n_rows: usize,
    n_cols: usize,
    from: (usize, Direction),
) -> u32 {
    let mut positions: Positions = Positions::new(n_rows, n_cols, from);

    while let Some((position, from)) = positions.pop() {
        match (from, map[position].1) {
            (Direction::Up, [true, _, _, _]) => continue,
            (Direction::Down, [_, true, _, _]) => continue,
            (Direction::Left, [_, _, true, _]) => continue,
            (Direction::Right, [_, _, _, true]) => continue,
            (_, _) => (),
        }

        match from {
            Direction::Up => map[position].1[0] = true,
            Direction::Down => map[position].1[1] = true,
            Direction::Left => map[position].1[2] = true,
            Direction::Right => map[position].1[3] = true,
        }

        match (map[position].0, from) {
            (Tile::Empty, Direction::Up) => positions.push_down(position),
            (Tile::Empty, Direction::Down) => positions.push_up(position),
            (Tile::Empty, Direction::Left) => positions.push_right(position),
            (Tile::Empty, Direction::Right) => positions.push_left(position),
            (Tile::MirrorUp, Direction::Up) => positions.push_left(position),
            (Tile::MirrorUp, Direction::Down) => positions.push_right(position),
            (Tile::MirrorUp, Direction::Left) => positions.push_up(position),
            (Tile::MirrorUp, Direction::Right) => positions.push_down(position),
            (Tile::MirrorDown, Direction::Up) => positions.push_right(position),
            (Tile::MirrorDown, Direction::Down) => positions.push_left(position),
            (Tile::MirrorDown, Direction::Left) => positions.push_down(position),
            (Tile::MirrorDown, Direction::Right) => positions.push_up(position),
            (Tile::SplitterVertical, Direction::Up) => positions.push_down(position),
            (Tile::SplitterVertical, Direction::Down) => positions.push_up(position),
            (Tile::SplitterVertical, Direction::Left) => {
                positions.push_up(position);
                positions.push_down(position);
            }
            (Tile::SplitterVertical, Direction::Right) => {
                positions.push_up(position);
                positions.push_down(position);
            }
            (Tile::SplitterHorizontal, Direction::Up) => {
                positions.push_left(position);
                positions.push_right(position);
            }
            (Tile::SplitterHorizontal, Direction::Down) => {
                positions.push_left(position);
                positions.push_right(position);
            }
            (Tile::SplitterHorizontal, Direction::Left) => positions.push_right(position),
            (Tile::SplitterHorizontal, Direction::Right) => positions.push_left(position),
        }
    }

    let mut total: u32 = 0;

    for (_, energized) in map {
        if energized[0] | energized[1] | energized[2] | energized[3] {
            total += 1;
        }
    }

    total
}
