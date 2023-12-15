fn main() {
    assert_eq!(part_1("HASH"), 52);
    assert_eq!(
        part_1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        1320
    );
    assert_eq!(
        part_1(&std::fs::read_to_string("input.txt").unwrap()),
        512_797
    );

    assert_eq!(
        part_2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        145
    );

    assert_eq!(
        part_2(&std::fs::read_to_string("input.txt").unwrap()),
        262_454
    );
}

fn part_1(input: &str) -> u32 {
    let mut total: u32 = 0;

    for line in input.split(',') {
        let mut value: u32 = 0;
        for c in line.chars() {
            value = (value + c as u32) * 17 % 256;
        }
        total += value;
    }

    println!("{total}");
    total
}

fn part_2(input: &str) -> u32 {
    let boxes = parse_input(input);

    let total = compute_power(&boxes);

    println!("{total}");
    total
}

#[derive(Debug, Clone)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    fn get(&self, label: &str) -> Option<usize> {
        for (i, lens) in self.lenses.iter().enumerate() {
            if lens.label == label {
                return Some(i);
            }
        }
        None
    }

    fn remove(&mut self, label: &str) {
        for (i, lens) in self.lenses.iter().enumerate() {
            if lens.label == label {
                self.lenses.remove(i);
                return;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    value: u32,
}

impl Lens {
    fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.parse::<u32>().unwrap(),
        }
    }

    fn replace(&mut self, value: &str) {
        self.value = value.parse::<u32>().unwrap();
    }
}

fn parse_input(input: &str) -> Vec<Box> {
    let mut boxes: Vec<Box> = vec![Box::new(); 256];

    for line in input.split(',') {
        if line.contains('=') {
            let (label, value) = line.split_once('=').unwrap();
            let hash = hash(label);
            if let Some(index) = boxes[hash].get(label) {
                boxes[hash].lenses[index].replace(value);
            } else {
                boxes[hash].lenses.push(Lens::new(label, value));
            }
        } else {
            let label = line.split_once('-').unwrap().0;
            let hash = hash(label);
            boxes[hash].remove(label);
        }
    }

    boxes
}

fn hash(label: &str) -> usize {
    label
        .chars()
        .map(|c| c as u32)
        .fold(0, |x, y| (x + y) * 17 % 256) as usize
}

fn compute_power(boxes: &[Box]) -> u32 {
    let mut total: u32 = 0;

    for (i, box_i) in boxes.iter().enumerate() {
        for (j, lens) in box_i.lenses.iter().enumerate() {
            total += u32::try_from((i + 1) * (j + 1)).unwrap() * lens.value;
        }
    }

    total
}
