use std::fmt::Display;

fn main() {
    assert_eq!(
        part_1(
            "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
            
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            
            temperature-to-humidity map:
            0 69 1
            1 0 69
            
            humidity-to-location map:
            60 56 37
            56 93 4"
        ),
        35
    );

    assert_eq!(
        part_2(
            "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
            
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            
            temperature-to-humidity map:
            0 69 1
            1 0 69
            
            humidity-to-location map:
            60 56 37
            56 93 4"
        ),
        46
    );
}

fn part_1(input: &str) -> usize {
    let (seeds, maps) = parse_input(input);

    let mut locations: Vec<usize> = Vec::new();

    for seed in seeds {
        let mut index = seed;

        for map in &maps {
            index = map.replace(index);
        }

        locations.push(index);
    }

    println!("min: {}", locations.iter().min().unwrap());

    *locations.iter().min().unwrap()
}

fn part_2(input: &str) -> usize {
    let (seeds, maps) = parse_input(input);

    let mut locations: Vec<usize> = Vec::new();

    let mut seeds_iter = seeds.iter();

    while let Some(start) = seeds_iter.next() {
        let length = seeds_iter.next().unwrap();

        for seed in *start..(start + length) {
            let mut index = seed;

            for map in &maps {
                index = map.replace(index);
            }

            locations.push(index);
        }
    }

    println!("min: {}", locations.iter().min().unwrap());

    *locations.iter().min().unwrap()
}

struct Map {
    replacements: Vec<Replacement>,
}

impl Map {
    fn new() -> Self {
        Self {
            replacements: Vec::new(),
        }
    }

    fn replace(&self, value: usize) -> usize {
        for replacement in &self.replacements {
            if replacement.includes(value) {
                return replacement.replace(value);
            }
        }

        value
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = "".into();

        for replacement in &self.replacements {
            string = format!("{}\n{}", string, replacement)
        }

        write!(f, "{string}")
    }
}

struct Replacement {
    source: usize,
    destination: usize,
    length: usize,
}

impl Replacement {
    fn includes(&self, value: usize) -> bool {
        (value >= self.source) & (value < self.source + self.length)
    }

    fn replace(&self, value: usize) -> usize {
        self.destination + value - self.source
    }
}

impl Display for Replacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}, {}",
            self.source, self.destination, self.length
        )
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Map>) {
    let mut seeds: Vec<usize> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();

    for line in input.lines() {
        if line.trim().starts_with("seeds: ") {
            seeds = line
                .replace("seeds: ", "")
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            continue;
        }

        if line.trim().starts_with(char::is_alphabetic) {
            maps.push(Map::new());
            continue;
        }

        if line.trim() == "" {
            continue;
        }

        let mut splitted = line.trim().splitn(3, ' ');
        let destination = splitted.next().unwrap().parse::<usize>().unwrap();
        let source = splitted.next().unwrap().parse::<usize>().unwrap();
        let length = splitted.next().unwrap().parse::<usize>().unwrap();

        maps.last_mut().unwrap().replacements.push(Replacement {
            source,
            destination,
            length,
        });
    }

    (seeds, maps)
}
