use std::fmt::Display;

const LINE_SIZE: usize = 10;

fn main() {
    assert_eq!(
        part_1(
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
        ),
        4361
    );

    assert_eq!(
        part_2(
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
        ),
        467835
    );
}

#[derive(Clone, Copy)]
struct Number {
    start: usize,
    end: usize,
    value: u32,
    is_part: bool,
}

impl Number {
    fn new(start: usize, end: usize, value: &str) -> Self {
        Self {
            start,
            end,
            value: value.parse::<u32>().unwrap(),
            is_part: false,
        }
    }

    fn check_part(&mut self, symbols: &[usize]) {
        let start_modulo = self.start % LINE_SIZE;
        let end_modulo = self.end % LINE_SIZE;

        let left: usize = if start_modulo == 0 {
            self.start
        } else {
            self.start - 1
        };

        let right: usize = if end_modulo == LINE_SIZE - 1 {
            self.end
        } else {
            self.end + 1
        };

        // Line before
        if self.start > LINE_SIZE {
            let start = left - LINE_SIZE;
            let end = right - LINE_SIZE;

            for i in start..=end {
                if symbols.contains(&i) {
                    self.is_part = true;
                    return;
                }
            }
        }

        // Just before
        if start_modulo != 0 && symbols.contains(&(self.start - 1)) {
            self.is_part = true;
            return;
        }

        // Just after
        if end_modulo != LINE_SIZE - 1 && symbols.contains(&(self.end + 1)) {
            self.is_part = true;
            return;
        }

        // Line after
        let start = left + LINE_SIZE;
        let end = right + LINE_SIZE;

        for i in start..=end {
            if symbols.contains(&i) {
                self.is_part = true;
                return;
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}, {}) {}",
            self.value, self.start, self.end, self.is_part
        )
    }
}

struct Star {
    position: usize,
    numbers: Vec<Number>,
    is_gear: bool,
    ratio: u32,
}

impl Star {
    fn new(position: usize) -> Self {
        Self {
            position,
            numbers: Vec::new(),
            is_gear: false,
            ratio: 0,
        }
    }

    fn check_gear(&mut self, numbers: Vec<Number>) {
        let modulo = self.position % LINE_SIZE;

        let left: usize = if modulo == 0 {
            self.position
        } else {
            self.position - 1
        };

        let right: usize = if modulo == LINE_SIZE - 1 {
            self.position
        } else {
            self.position + 1
        };

        for number in numbers {
            if self.position > LINE_SIZE {
                let start = left - LINE_SIZE;
                let end = right - LINE_SIZE;

                if ((number.start >= start) & (number.start <= end))
                    | ((number.end >= start) & (number.end <= end))
                    | ((number.start < start) & (number.end > end))
                {
                    self.numbers.push(number);
                    continue;
                }
            }

            if self.position % LINE_SIZE != 0 && number.end == self.position - 1 {
                self.numbers.push(number);
                continue;
            }

            if self.position % LINE_SIZE != LINE_SIZE - 1 && number.start == self.position + 1 {
                self.numbers.push(number);
                continue;
            }

            let start = left + LINE_SIZE;
            let end = right + LINE_SIZE;

            if ((number.start >= start) & (number.start <= end))
                | ((number.end >= start) & (number.end <= end))
                | ((number.start < start) & (number.end > end))
            {
                self.numbers.push(number);
                continue;
            }
        }

        if self.numbers.len() == 2 {
            self.is_gear = true;
            self.ratio = self.numbers.get(0).unwrap().value * self.numbers.get(1).unwrap().value
        }
    }
}

impl Display for Star {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) = {}", self.position, self.ratio)
    }
}

fn part_1(input: &str) -> u32 {
    // Parse numbers and symbols
    let mut number_list: Vec<Number> = Vec::new();
    let mut symbol_list: Vec<usize> = Vec::new();

    let mut start: Option<usize> = None;
    let mut position: usize = 0;
    let mut spaces: usize = 0;

    for (i, c) in input.char_indices() {
        match c {
            ' ' | '\t' => spaces += 1,
            '0'..='9' => {
                match start {
                    Some(_) => (),
                    None => start = Some(position),
                }
                position += 1;
            }
            '.' => {
                if let Some(x) = start {
                    number_list.push(Number::new(x, position - 1, &input[(x + spaces)..i]));
                    start = None
                }
                position += 1;
            }
            '\n' => {
                if let Some(x) = start {
                    number_list.push(Number::new(x, position - 1, &input[(x + spaces)..i]));
                    start = None
                }
                spaces += 1;
            }
            _ => {
                if let Some(x) = start {
                    number_list.push(Number::new(x, position - 1, &input[(x + spaces)..i]));
                    start = None
                }
                symbol_list.push(position);
                position += 1;
            }
        }
    }

    // Check for each number if adhacent symbol
    let mut total = 0;

    for mut number in number_list {
        number.check_part(&symbol_list);
        if number.is_part {
            total += number.value;
        }
    }

    println!("{total}");
    total
}

fn part_2(input: &str) -> u32 {
    // Parse numbers and symbols
    let mut number_list: Vec<Number> = Vec::new();
    let mut gear_list: Vec<Star> = Vec::new();

    let mut start: Option<usize> = None;
    let mut position: usize = 0;
    let mut spaces: usize = 0;

    for (i, c) in input.char_indices() {
        match c {
            ' ' | '\t' => spaces += 1,
            '0'..='9' => {
                match start {
                    Some(_) => (),
                    None => start = Some(position),
                }
                position += 1;
            }
            '*' => {
                if let Some(x) = start {
                    number_list.push(Number::new(x, position - 1, &input[(x + spaces)..i]));
                    start = None
                }
                gear_list.push(Star::new(position));
                position += 1;
            }
            '\n' => {
                if let Some(x) = start {
                    number_list.push(Number::new(x, position - 1, &input[(x + spaces)..i]));
                    start = None
                }
                spaces += 1;
            }
            _ => {
                if let Some(x) = start {
                    number_list.push(Number::new(x, position - 1, &input[(x + spaces)..i]));
                    start = None
                }
                position += 1;
            }
        }
    }

    // Check for each number if adhacent symbol
    let mut total = 0;

    for mut star in gear_list {
        star.check_gear(number_list.clone());
        if star.is_gear {
            total += star.ratio;
        }
    }

    println!("{total}");
    total
}
