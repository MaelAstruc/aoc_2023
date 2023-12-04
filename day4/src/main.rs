fn main() {
    assert_eq!(
        part_1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        ),
        13
    );

    assert_eq!(
        part_2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        ),
        30
    );
}

fn part_1(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let n = parse_line(line);
        if n > 0 {
            total += 2_u32.pow((n - 1).into());
        }
    }
    println!("{total}");
    total
}

fn part_2(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut copies: Vec<u32> = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let n = parse_line(line);
        for j in 1..=n.into() {
            copies[i + j] += copies[i];
        }
        total += copies[i];
    }
    println!("{total}");
    total
}

fn parse_line(line: &str) -> u16 {
    let line_clean = line.trim().replace("  ", " ");
    let (_card, numbers) = line_clean.split_once(':').unwrap();
    let (winning_str, have_str) = numbers.split_once('|').unwrap();
    let winning: Vec<u16> = winning_str
        .trim()
        .split(' ')
        .map(|n| n.trim().parse::<u16>().unwrap())
        .collect();
    let have: Vec<u16> = have_str
        .trim()
        .split(' ')
        .map(|n| n.parse::<u16>().unwrap())
        .collect();
    let mut n: u16 = 0;
    for x in have {
        if winning.contains(&x) {
            n += 1;
        }
    }
    n
}
