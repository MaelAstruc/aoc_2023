fn main() {
    assert_eq!(
        part1(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"
        ),
        142
    );

    assert_eq!(
        part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"
        ),
        281
    );

    assert_eq!(
        part2_v2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"
        ),
        281
    );
}

fn part1(input: &str) -> u32 {
    let output: Vec<String> = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect())
        .collect();

    let result: u32 = output
        .iter()
        .map(|string| {
            format!(
                "{}{}",
                string.chars().next().unwrap_or('0'),
                string.chars().last().unwrap_or('0')
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum();
    println!("{result}");
    result
}

fn part2(input: &str) -> u32 {
    let output: Vec<String> = input
        .lines()
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect()
        })
        .collect();

    let result: u32 = output
        .iter()
        .map(|string| {
            format!(
                "{}{}",
                string.chars().next().unwrap_or('0'),
                string.chars().last().unwrap_or('0')
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum();
    println!("{result}");
    result
}

fn part2_v2(input: &str) -> u32 {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;
    let mut total: u32 = 0;

    for (i, c) in input.char_indices() {
        match c {
            '1'..='9' => {
                (first, last) = update(first, last, c.to_string().parse::<u32>().unwrap())
            }
            'o' => {
                if check_number_name(input, i, "one") {
                    (first, last) = update(first, last, 1)
                }
            }
            't' => {
                if check_number_name(input, i, "two")  {
                    (first, last) = update(first, last, 2)
                }
                else if check_number_name(input, i, "three")  {
                    (first, last) = update(first, last, 3)
                }
            }
            'f' => {
                if check_number_name(input, i, "four") {
                    (first, last) = update(first, last, 4)
                }
                else if check_number_name(input, i, "five")  {
                    (first, last) = update(first, last, 5)
                }
            }
            's' => {
                if check_number_name(input, i, "six") {
                    (first, last) = update(first, last, 6)
                }
                else if check_number_name(input, i, "seven") {
                    (first, last) = update(first, last, 7)
                }
            }
            'e' => {
                if check_number_name(input, i, "eight") {
                    (first, last) = update(first, last, 8)
                }
            }
            'n' => {
                if check_number_name(input, i, "nine") {
                    (first, last) = update(first, last, 9)
                }
            }
            '\n' => {
                total += new_value(first, last);
                (first, last) = (None, None)
            }
            _ => (),
        }
    }
    
    // For the last line
    total += new_value(first, last);

    println!("{total}");
    total
}

fn update(first: Option<u32>, last: Option<u32>, value: u32) -> (Option<u32>, Option<u32>) {
    match (first, last) {
        (Some(_), Some(_)) => (first, Some(value)),
        (None, None) => (Some(value), Some(value)),
        (Some(_), None) => unreachable!(),
        (None, Some(_)) => unreachable!(),
    }
}

fn check_number_name(input: &str, i: usize, name: &str) -> bool{
    i + name.len() <= input.len() && &input[i..(i+name.len())] == name
}

fn new_value(first: Option<u32>, last: Option<u32>) -> u32 {
    match (first, last) {
        (Some(x), Some(y)) => x * 10 + y,
        (None, None) => 0,
        (Some(_), None) => unreachable!(),
        (None, Some(_)) => unreachable!(),
    }
}
