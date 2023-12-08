use std::collections::HashMap;

fn main() {
    assert_eq!(
        part_1(
            "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"
        ),
        2
    );

    assert_eq!(
        part_1(
            "LLR
    
        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
        ),
        6
    );

    assert_eq!(
        part_2(
            "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"
        ),
        6
    );
}

fn part_2(input: &str) -> usize {
    let (instructions, starts, graph) = parse_input(input);

    // Assume that there are only loops with 1 count
    let mut counts: Vec<usize> = Vec::new();

    for start in &starts {
        let mut i: usize = 0;
        let mut graph_count: GraphCount = HashMap::new();

        let mut id = start;
        let mut last_main = start;
        let mut steps: usize = 0;

        loop {
            if i == instructions.len() {
                i = 0;
            }

            let instruction = instructions.get(i).unwrap();
            let node = graph.get(id).unwrap();

            steps += 1;
            i += 1;

            id = match &instruction {
                Instruction::Left => &node.0,
                Instruction::Right => &node.1,
            };

            if id.ends_with('Z') {
                match graph_count.insert(last_main.to_string(), (id.to_string(), steps)) {
                    Some(_) => break,
                    None => {
                        last_main = id;
                        steps = 0;
                    }
                };
            }
        }

        for (_old, (_new, count)) in &graph_count {
            // println!("\t{old} => {new} in {count}");
            // Assume one cycle possible
            counts.push(*count)
        }
    }

    let lcm = lcm(&counts);

    println!("{lcm}");

    lcm
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn part_1(input: &str) -> u32 {
    let (instructions, _starts, graph) = parse_input(input);

    let mut id = "AAA".to_string();
    let mut i: usize = 0;
    let mut steps: u32 = 0;

    loop {
        if i == instructions.len() {
            i = 0;
        }

        let instruction = instructions.get(i).unwrap();
        let node = graph.get(&id).unwrap();

        steps += 1;
        i += 1;

        id = match instruction {
            Instruction::Left => node.0.clone(),
            Instruction::Right => node.1.clone(),
        };

        if id == "ZZZ" {
            break;
        }
    }

    println!("{steps}");
    steps
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

type Graph = HashMap<String, (String, String)>;

type GraphCount = HashMap<String, (String, usize)>;

fn parse_input(input: &str) -> (Vec<Instruction>, Vec<String>, Graph) {
    let mut input_lines = input.lines();

    let mut instructions: Vec<Instruction> = Vec::new();

    let mut starts: Vec<String> = Vec::new();

    for char in input_lines.next().unwrap().chars() {
        match char {
            'L' => instructions.push(Instruction::Left),
            'R' => instructions.push(Instruction::Right),
            _ => unreachable!(),
        }
    }

    let mut graph: HashMap<String, (String, String)> = HashMap::new();

    for line in input_lines {
        if line.trim().is_empty() {
            continue;
        }

        let (id, possibilities) = line.split_once(" = ").unwrap();
        let clean = possibilities.replace(['(', ')'], "");
        let tuple_split = clean.split_once(", ").unwrap();
        let tuple = (tuple_split.0.to_string(), tuple_split.1.to_string());
        graph.insert(id.trim().to_string(), tuple);

        if id.trim().ends_with('A') {
            starts.push(id.trim().to_string());
        }
    }

    (instructions, starts, graph)
}
