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
    )
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
