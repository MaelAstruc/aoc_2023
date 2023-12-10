fn main() {
    assert_eq!(
        part_1(
            "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"
        ),
        114
    );

    assert_eq!(
        part_2(
            "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"
        ),
        2
    );
}

fn part_1(input: &str) -> i32 {
    let total: i32 = input
        .lines()
        .map(|line| predict_line(line, &Direction::Forward))
        .sum();

    println!("{total}");
    total
}

fn part_2(input: &str) -> i32 {
    let total: i32 = input
        .lines()
        .map(|line| predict_line(line, &Direction::Backward))
        .sum();

    println!("{total}");
    total
}

enum Direction {
    Forward,
    Backward,
}

fn predict_line(line: &str, direction: &Direction) -> i32 {
    let vector: Vec<i32> = line
        .trim()
        .split_ascii_whitespace()
        .map(|value| value.parse::<i32>().unwrap())
        .collect();

    predict(&vector, direction)
}

fn predict(vector: &[i32], direction: &Direction) -> i32 {
    if is_zeros(vector) {
        return 0;
    }

    let mut difference: Vec<i32> = Vec::with_capacity(vector.len() - 1);

    for i in 0..(vector.len() - 1) {
        difference.push(vector[i + 1] - vector[i]);
    }

    let prediction = predict(&difference, direction);

    match direction {
        Direction::Forward => vector.last().unwrap() + prediction,
        Direction::Backward => vector.first().unwrap() - prediction,
    }
}

fn is_zeros(vector: &[i32]) -> bool {
    for x in vector {
        if x != &0i32 {
            return false;
        }
    }
    true
}
