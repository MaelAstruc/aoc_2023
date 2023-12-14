use std::collections::HashMap;

fn main() {
    assert_eq!(
        part_1(
            "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        ),
        136
    );

    assert_eq!(
        part_2(
            "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        ),
        64
    );
}

fn part_1(input: &str) -> usize {
    let n_cols = input.lines().next().unwrap().len();
    let n_rows = input.lines().count();

    let mut positions: Vec<usize> = vec![0; n_cols];
    let mut loads: Vec<usize> = vec![0; n_cols];

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            match c {
                'O' => {
                    loads[col] += n_rows - positions[col];
                    positions[col] += 1;
                }
                '#' => positions[col] = row + 1,
                '.' => (),
                _ => unreachable!("Unexpected character {c}"),
            }
        }
    }

    let total: usize = loads.iter().sum();
    println!("{total}");
    total
}

fn part_2(input: &str) -> usize {
    let n_cols = input.lines().next().unwrap().len();
    let n_rows = input.lines().count();

    let mut shapes: Vec<Shape> = parse_input(input);

    roll_n(&mut shapes, n_rows, n_cols, 1_000_000_000);

    let north_load = count_load(&shapes, n_rows, n_cols);

    println!("{north_load}");
    north_load
}

fn roll_n(shapes: &mut Vec<Shape>, n_rows: usize, n_cols: usize, n_rolls: usize) -> Vec<Shape> {
    let mut cache: HashMap<Vec<Shape>, Vec<Shape>> = HashMap::new();
    let mut cycle: usize = 0;

    while cycle < n_rolls {
        // We cache the results and check if we already tried it
        if cache.contains_key(shapes) {
            let mut last_result = shapes.clone();
            let mut in_cycles: usize = 0;

            // While we can find result we tried we continue and count cycles
            while let Some(new_result) = cache.get(&last_result) {
                in_cycles += 1;

                // If we loop over the initial input, we can skip to almost the end
                if new_result == shapes {
                    let distance = in_cycles;
                    let remaining = n_rolls - cycle;
                    let possible = remaining / distance; // The division of usize is the integer
                    in_cycles = possible * distance;
                }

                last_result = new_result.clone();

                if cycle + in_cycles == n_rolls {
                    break;
                }
            }

            cycle += in_cycles;

            *shapes = last_result;
        } else {
            let copy = shapes.clone();

            roll_north(shapes, n_rows, n_cols);
            roll_west(shapes, n_rows, n_cols);
            roll_south(shapes, n_rows, n_cols);
            roll_east(shapes, n_rows, n_cols);

            cache.insert(copy, shapes.clone());

            cycle += 1;
        }
    }

    shapes.clone()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Shape {
    Round,
    Cube,
    Empty,
}

fn parse_input(input: &str) -> Vec<Shape> {
    let mut map: Vec<Shape> = Vec::new();

    for c in input.chars() {
        match c {
            ' ' | '\t' | '\n' => (),
            'O' => map.push(Shape::Round),
            '#' => map.push(Shape::Cube),
            '.' => map.push(Shape::Empty),
            _ => unreachable!("Unexpected character {c}"),
        }
    }

    map
}

fn count_load(input: &[Shape], n_rows: usize, n_cols: usize) -> usize {
    let mut load = 0;

    for row in 0..n_rows {
        for col in 0..n_cols {
            let index = row * n_cols + col;
            match input[index] {
                Shape::Round => load += n_rows - row,
                Shape::Cube | Shape::Empty => (),
            }
        }
    }

    load
}

fn roll_north(input: &mut [Shape], n_rows: usize, n_cols: usize) {
    for col in 0..n_cols {
        let mut last_empty = col;

        for row in 0..n_rows {
            let index = row * n_cols + col;
            match input[index] {
                Shape::Round => {
                    input.swap(last_empty, index);
                    last_empty += n_cols;
                }
                Shape::Cube => last_empty = index + n_cols,
                Shape::Empty => (),
            }
        }
    }
}

fn roll_west(input: &mut [Shape], n_rows: usize, n_cols: usize) {
    for row in 0..n_rows {
        let mut last_empty = row * n_cols;

        for col in 0..n_cols {
            let index = row * n_cols + col;
            match input[index] {
                Shape::Round => {
                    input.swap(last_empty, index);
                    last_empty += 1;
                }
                Shape::Cube => last_empty = index + 1,
                Shape::Empty => (),
            }
        }
    }
}

fn roll_south(input: &mut [Shape], n_rows: usize, n_cols: usize) {
    for col in 0..n_cols {
        let mut last_empty = (n_rows - 1) * n_cols + col;

        for row in (0..n_rows).rev() {
            let index = row * n_cols + col;
            match input[index] {
                Shape::Round => {
                    input.swap(last_empty, index);

                    if row > 0 {
                        last_empty -= n_cols;
                    }
                }
                Shape::Cube => {
                    if row > 0 {
                        last_empty = index - n_cols;
                    }
                }
                Shape::Empty => (),
            }
        }
    }
}

fn roll_east(input: &mut [Shape], n_rows: usize, n_cols: usize) {
    for row in 0..n_rows {
        let mut last_empty = row * n_cols + n_cols - 1;

        for col in (0..n_cols).rev() {
            let index = row * n_cols + col;
            match input[index] {
                Shape::Round => {
                    input.swap(last_empty, index);
                    last_empty -= 1;
                }
                Shape::Cube => last_empty = index - 1,
                Shape::Empty => (),
            }
        }
    }
}
