pub(crate) fn part_1(input: &str, steps: usize) {
    let (mut map, start, n_row, n_col) = parse_input(input);

    let mut positions: Vec<usize> = Vec::new();
    positions.push(start);

    for _ in 0..steps {
        let mut new_positions: Vec<usize> = Vec::new();
        while let Some(position) = positions.pop() {
            let distance = match map.get(position).unwrap() {
                Tile::Empty(x) => x.unwrap(),
                Tile::Rock => unreachable!("Shouldn't start from a rock in {position}"),
            };
            if position >= n_col {
                let index = position - n_col;
                match map.get(index).unwrap() {
                    Tile::Empty(None) => {
                        map[index] = Tile::Empty(Some(distance + 1));
                        new_positions.push(index);
                    }
                    Tile::Empty(Some(_)) => (),
                    Tile::Rock => (),
                }
            }
            if position < (n_row - 1) * n_col {
                let index = position + n_col;
                match map.get(index).unwrap() {
                    Tile::Empty(None) => {
                        map[index] = Tile::Empty(Some(distance + 1));
                        new_positions.push(index);
                    }
                    Tile::Empty(Some(_)) => (),
                    Tile::Rock => (),
                }
            }
            if position % n_col != 0 {
                let index = position - 1;
                match map.get(index).unwrap() {
                    Tile::Empty(None) => {
                        map[index] = Tile::Empty(Some(distance + 1));
                        new_positions.push(index);
                    }
                    Tile::Empty(Some(_)) => (),
                    Tile::Rock => (),
                }
            }
            if position % n_col != n_col - 1 {
                let index = position + 1;
                match map.get(index).unwrap() {
                    Tile::Empty(None) => {
                        map[index] = Tile::Empty(Some(distance + 1));
                        new_positions.push(index);
                    }
                    Tile::Empty(Some(_)) => (),
                    Tile::Rock => (),
                }
            }
        }
        new_positions.sort_unstable();
        new_positions.dedup();
        positions = new_positions;
    }

    let mut total: usize = 0;

    for tile in map {
        match tile {
            Tile::Empty(None) => (),
            Tile::Empty(Some(distance)) => {
                if distance % 2 == 0 {
                    total += 1;
                }
            }
            Tile::Rock => (),
        }
    }

    println!("{total}");
}

#[derive(Debug)]
enum Tile {
    Empty(Option<usize>),
    Rock,
}

fn parse_input(input: &str) -> (Vec<Tile>, usize, usize, usize) {
    let mut map: Vec<Tile> = Vec::new();
    let mut start: Option<usize> = None;

    let n_row = input.lines().count();
    let n_col = input.lines().next().unwrap().len();

    for line in input.lines() {
        for c in line.trim().chars() {
            match c {
                '.' => map.push(Tile::Empty(None)),
                '#' => map.push(Tile::Rock),
                'S' => {
                    start = Some(map.len());
                    map.push(Tile::Empty(Some(0)));
                }
                _ => unreachable!("Unexpected character {c}"),
            }
        }
    }

    (map, start.unwrap(), n_row, n_col)
}
