fn main() {
    assert_eq!(
        part_1(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#....."
        ),
        374
    );

    assert_eq!(
        part_2(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
            2
        ),
        374
    );

    assert_eq!(
        part_2(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
            10
        ),
        1030
    );

    assert_eq!(
        part_2(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
            100
        ),
        8410
    );
}

fn part_1(input: &str) -> usize {
    let (stars, empty_rows, empty_cols) = parse_input(input);

    let rows_expansion = expand_empty(&empty_rows, 2);
    let cols_expansion = expand_empty(&empty_cols, 2);

    let stars_expanded = expand_stars(&stars, &rows_expansion, &cols_expansion);

    let total = compute_distances(&stars_expanded);

    println!("{total}");

    total
}

fn part_2(input: &str, expansion_factor: usize) -> usize {
    let (stars, empty_rows, empty_cols) = parse_input(input);

    let rows_expansion = expand_empty(&empty_rows, expansion_factor);
    let cols_expansion = expand_empty(&empty_cols, expansion_factor);

    let stars_expanded = expand_stars(&stars, &rows_expansion, &cols_expansion);

    let total = compute_distances(&stars_expanded);

    println!("{total}");

    total
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<bool>, Vec<bool>) {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().unwrap().trim().chars().count();

    let mut empty_rows = vec![true; n_rows];
    let mut empty_cols = vec![true; n_cols];

    let mut stars: Vec<(usize, usize)> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    empty_rows[i] = false;
                    empty_cols[j] = false;
                    stars.push((i, j));
                }
                _ => unreachable!(),
            }
        }
    }

    (stars, empty_rows, empty_cols)
}

fn expand_empty(empty_vec: &[bool], expansion_factor: usize) -> Vec<usize> {
    let mut vec_expansion = vec![0; empty_vec.len()];

    let mut count = 0;

    for i in 0..empty_vec.len() {
        if empty_vec[i] {
            count += expansion_factor - 1;
        }
        vec_expansion[i] = i + count;
    }

    vec_expansion
}

fn expand_stars(
    stars: &[(usize, usize)],
    rows_expansion: &[usize],
    cols_expansion: &[usize],
) -> Vec<(usize, usize)> {
    let mut stars_expanded = vec![(0, 0); stars.len()];

    for (i, (row, col)) in stars.iter().enumerate() {
        stars_expanded[i] = (rows_expansion[*row], cols_expansion[*col]);
    }

    stars_expanded
}

fn compute_distances(stars: &[(usize, usize)]) -> usize {
    let mut total = 0;

    for (i, (row_1, col_1)) in stars.iter().enumerate() {
        for (row_2, col_2) in &stars[(i + 1)..] {
            total += row_1.abs_diff(*row_2) + col_1.abs_diff(*col_2);
        }
    }

    total
}
