fn main() {
    assert_eq!(
        part_1(
            "1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9"
        ),
        5
    );

    assert_eq!(
        part_2(
            "1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9"
        ),
        7
    );
}

fn part_1(input: &str) -> usize {
    let mut bricks = parse_input(input);

    fall(&mut bricks);

    let mut cannot_drop: Vec<usize> = Vec::new();

    for (i, brick) in bricks.iter().enumerate() {
        let mut supporters: Vec<usize> = Vec::new();

        for (j, other_brick) in bricks[..i].iter().enumerate() {
            if other_brick.supports(brick) {
                supporters.push(j);
            }
        }

        if supporters.len() == 1 {
            cannot_drop.push(supporters[0]);
        }
    }

    cannot_drop.sort_unstable();
    cannot_drop.dedup();

    let can_drop_count = bricks.len() - cannot_drop.len();

    println!("{can_drop_count}");

    can_drop_count
}

fn part_2(input: &str) -> usize {
    let mut bricks = parse_input(input);

    fall(&mut bricks);

    let mut fall_count: Vec<usize> = Vec::new();

    for (root_index, _) in bricks.iter().enumerate() {
        let are_supported: Vec<usize> = get_all_supported(&bricks, root_index);

        let mut would_fall: Vec<usize> = vec![];

        for brick_index in are_supported {
            if find_root(&bricks, brick_index, root_index) {
                would_fall.push(brick_index);
            }
        }

        would_fall.dedup();

        fall_count.push(would_fall.len());
    }

    let total_fall_count = fall_count.iter().sum::<usize>();

    println!("{total_fall_count}");

    total_fall_count
}

fn get_all_supported(bricks: &[Brick], index: usize) -> Vec<usize> {
    let mut are_supported: Vec<usize> = vec![];

    let brick = bricks.get(index).unwrap();

    for (other_index, other_brick) in bricks[(index + 1)..].iter().enumerate() {
        if other_brick.start.z > brick.end.z + 1 {
            break;
        }
        if brick.supports(other_brick) {
            are_supported.push(index + other_index + 1);
            are_supported.append(&mut get_all_supported(bricks, index + other_index + 1));
        }
    }

    are_supported.sort_unstable();
    are_supported.dedup();

    are_supported
}

fn find_root(bricks: &[Brick], brick_index: usize, root_index: usize) -> bool {
    let brick = bricks.get(brick_index).unwrap();

    let mut supporters: Vec<usize> = Vec::new();

    for (other_index, other_brick) in bricks[..brick_index].iter().enumerate().rev() {
        if other_brick.end.z < brick.start.z - 1 {
            continue;
        }
        if other_brick.supports(brick) {
            supporters.push(other_index);
        }
    }

    if supporters.is_empty() {
        return false;
    }
    if supporters.len() == 1 && supporters[0] == root_index {
        return true;
    }
    if supporters.contains(&root_index) {
        return false;
    }

    supporters
        .iter()
        .all(|index| find_root(bricks, *index, root_index))
}

fn fall(bricks: &mut Vec<Brick>) {
    bricks.sort();

    for i in 0..bricks.len() {
        let mut min_distance = None;

        for other_brick in &bricks[..i] {
            if let Some(distance) = bricks[i].distance_above(other_brick) {
                if let Some(current_min) = min_distance {
                    if current_min > distance {
                        min_distance = Some(distance);
                    }
                } else {
                    min_distance = Some(distance);
                }
            }
        }

        if let Some(distance) = min_distance {
            bricks[i].start.z -= distance;
            bricks[i].end.z -= distance;
        } else {
            let difference = bricks[i].end.z - bricks[i].start.z;
            bricks[i].start.z = 1;
            bricks[i].end.z = difference + 1;
        }
    }

    bricks.sort();
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
struct Brick {
    start: Point,
    end: Point,
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.start.partial_cmp(&other.start) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.end.partial_cmp(&other.end)
    }
}

impl Brick {
    fn new(input: &str) -> Self {
        let (start, end) = input.split_once('~').unwrap();
        Self {
            start: Point::new(start),
            end: Point::new(end),
        }
    }

    fn is_below(&self, brick: &Brick) -> bool {
        if self.end.z >= brick.start.z {
            return false;
        }

        if (self.start.x > brick.end.x) | (self.end.x < brick.start.x) {
            return false;
        }

        if (self.start.y > brick.end.y) | (self.end.y < brick.start.y) {
            return false;
        }

        true
    }

    fn is_above(&self, brick: &Brick) -> bool {
        if self.start.z <= brick.end.z {
            return false;
        }

        if (self.start.x > brick.end.x) | (self.end.x < brick.start.x) {
            return false;
        }

        if (self.start.y > brick.end.y) | (self.end.y < brick.start.y) {
            return false;
        }

        true
    }

    fn distance_above(&self, brick: &Brick) -> Option<u32> {
        if self.is_above(brick) {
            Some(self.start.z - brick.end.z - 1)
        } else {
            None
        }
    }

    fn distance_below(&self, brick: &Brick) -> Option<u32> {
        if self.is_below(brick) {
            Some(brick.start.z - self.end.z - 1)
        } else {
            None
        }
    }

    fn supports(&self, brick: &Brick) -> bool {
        match self.distance_below(brick) {
            Some(0) => true,
            Some(_) | None => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.z.partial_cmp(&other.z) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

impl Point {
    fn new(input: &str) -> Self {
        let mut points = input.split(',');
        let x = points.next().unwrap().parse::<u32>().unwrap();
        let y = points.next().unwrap().parse::<u32>().unwrap();
        let z = points.next().unwrap().parse::<u32>().unwrap();
        Self { x, y, z }
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();

    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        bricks.push(Brick::new(line));
    }

    bricks
}
