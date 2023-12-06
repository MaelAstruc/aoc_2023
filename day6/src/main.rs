use std::fmt::Display;

fn main() {
    assert_eq!(
        part_1(
            "Time:      7  15   30
            Distance:  9  40  200"
        ),
        288
    );

    assert_eq!(
        part_2(
            "Time:      7  15   30
            Distance:  9  40  200"
        ),
        71503
    );
}

fn part_1(input: &str) -> u64 {
    let races = parse_input(input);
    let mut number_ways: u64 = 1;

    for race in races.iter() {
        number_ways *= race.n_possibilities();
    }

    println!("{number_ways}");
    number_ways
}

fn part_2(input: &str) -> u64 {
    let race = parse_input_nospace(input);
    let number_ways: u64 = race.n_possibilities();

    println!("{number_ways}");
    number_ways
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn min_max(&self) -> Option<(u64, u64)> {
        if self.time.pow(2) < self.distance {
            return None;
        }

        let delta_sqrt = ((self.time.pow(2) - 4 * self.distance) as f64).sqrt();
        let min = (self.time as f64 - delta_sqrt) / 2.0 + 10.0 * std::f64::EPSILON;
        let max = (self.time as f64 + delta_sqrt) / 2.0 - 10.0 * std::f64::EPSILON;
        Some((min.ceil() as u64, max.floor() as u64))
    }

    fn n_possibilities(&self) -> u64 {
        match self.min_max() {
            Some((min, max)) => max - min + 1,
            None => 0,
        }
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mm in {}ms", self.distance, self.time)
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();

    let mut input_iter = input.lines();
    let times: Vec<u64> = input_iter
        .next()
        .unwrap()
        .trim()
        .replace("Time: ", "")
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = input_iter
        .next()
        .unwrap()
        .trim()
        .replace("Distance: ", "")
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for i in 0..times.len() {
        races.push(Race::new(times[i], distances[i]));
    }

    races
}

fn parse_input_nospace(input: &str) -> Race {
    let binding = input.replace(' ', "");
    let mut input_iter = binding.lines();
    let time: u64 = input_iter
        .next()
        .unwrap()
        .replace("Time:", "")
        .parse::<u64>()
        .unwrap();
    let distance: u64 = input_iter
        .next()
        .unwrap()
        .replace("Distance:", "")
        .parse::<u64>()
        .unwrap();

    Race::new(time, distance)
}
