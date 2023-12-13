fn main() {
    assert_eq!(
        part_1(
            "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#"
        ),
        405
    );

    assert_eq!(
        part_2(
            "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#"
        ),
        400
    );
}

fn part_1(input: &str) -> usize {
    let input_clean = input.replace(' ', "");

    let mut total: usize = 0;

    for block in input_clean.split("\n\n") {
        let (new_block, n_rows, n_cols) = parse_block(block);
        total += 100 * count_mirrors(&new_block, n_rows, n_cols).0;
        let mut transposed_block: Vec<u8> = vec![0; new_block.len()];
        transpose::transpose(&new_block, &mut transposed_block, n_cols, n_rows);
        total += count_mirrors(&transposed_block, n_cols, n_rows).0;
    }

    println!("{total}");
    total
}

fn part_2(input: &str) -> usize {
    let input_clean = input.replace(' ', "");

    let mut total: usize = 0;

    for block in input_clean.split("\n\n") {
        let (new_block, n_rows, n_cols) = parse_block(block);
        total += 100 * count_mirrors_smudged(&new_block, n_rows, n_cols).0;
        let mut transposed_block: Vec<u8> = vec![0; new_block.len()];
        transpose::transpose(&new_block, &mut transposed_block, n_cols, n_rows);
        total += count_mirrors_smudged(&transposed_block, n_cols, n_rows).0;
    }

    println!("{total}");
    total
}

fn parse_block(block: &str) -> (Vec<u8>, usize, usize) {
    let n_rows = block.lines().count();
    let n_cols = block.lines().next().unwrap().len();

    let new_block: Vec<u8> = block
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => unreachable!("Unexpected character {c}"),
        })
        .collect();

    (new_block, n_rows, n_cols)
}

fn count_mirrors(block: &[u8], n_rows: usize, n_cols: usize) -> (usize, usize) {
    let (mut before, mut after): (usize, usize) = (0, 0);

    for row in 1..n_rows {
        let max_distance = std::cmp::min(row, n_rows - row);
        let mut all_symetric: bool = true;

        for distance in 1..=max_distance {
            let row_1_start = (row - distance) * n_cols;
            let row_1_end = (row - distance + 1) * n_cols;
            let row_2_start = (row + distance - 1) * n_cols;
            let row_2_end = (row + distance) * n_cols;

            if block[row_1_start..row_1_end] != block[row_2_start..row_2_end] {
                all_symetric = false;
                break;
            }
        }

        if all_symetric {
            before += row;
            after += n_rows - row;
        }
    }

    (before, after)
}

fn count_mirrors_smudged(block: &[u8], n_rows: usize, n_cols: usize) -> (usize, usize) {
    let (mut before, mut after): (usize, usize) = (0, 0);

    for row in 1..n_rows {
        let max_distance = std::cmp::min(row, n_rows - row);

        let mut number_asymmetric: usize = 0;

        for distance in 1..=max_distance {
            let row_1_start = (row - distance) * n_cols;
            let row_2_start = (row + distance - 1) * n_cols;

            let mut number_differences: usize = 0;

            for i in 0..n_cols {
                if block[row_1_start + i] != block[row_2_start + i] {
                    number_differences += 1;
                }
            }

            number_asymmetric += match number_differences {
                0 => 0,
                1 => 1,
                _ => 2,
            };
        }

        if number_asymmetric == 1 {
            before += row;
            after += n_rows - row;
        }
    }

    (before, after)
}
