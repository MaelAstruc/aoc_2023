use crate::{Direction, Tile};

pub(crate) fn part_2(input: &str) -> u32 {
    let line_size = input.lines().next().unwrap().chars().count();

    let mut map: Vec<Tile> = Vec::new();
    for line in input.lines() {
        map.append(&mut parse_line(line))
    }

    // Find the starting tile
    let start = get_start(&map);

    // Find which tiles are in the loop
    let (in_loop, first_direction, last_direction) = solve_loop(&map, start, line_size);

    // Replace the start by the corresponding tile
    map[start] = Tile::from((last_direction.reverse(), first_direction));

    // Replace the tiles not in the loop by ground tiles
    for i in 0..map.len() {
        if !in_loop[i] {
            map[i] = Tile::Ground;
        }
    }

    // Count the ground tiles within the loop
    let total = count_within(&map, line_size);

    println!("{total}");
    total
}

fn parse_line(line: &str) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();

    for c in line.trim().chars() {
        let tile: Tile = match c {
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!("Unexpected character: {c}"),
        };

        tiles.push(tile);
    }

    tiles
}

fn find_departure(map: &[Tile], start: usize, line_size: usize) -> Direction {
    if start > line_size {
        if let Some(Tile::NS | Tile::SE | Tile::SW) = map.get(start - line_size) {
            return Direction::North;
        }
    }

    if start % line_size != 0 {
        if let Some(Tile::NE | Tile::SE | Tile::EW) = map.get(start - 1) {
            return Direction::West;
        }
    }

    if start % line_size != line_size {
        if let Some(Tile::NW | Tile::SW | Tile::EW) = map.get(start + 1) {
            return Direction::East;
        }
    }

    if let Some(Tile::NS | Tile::SE | Tile::SW) = map.get(start + line_size) {
        return Direction::South;
    }

    panic!("Cannot find a direction to start")
}

fn solve_loop(map: &[Tile], start: usize, line_size: usize) -> (Vec<bool>, Direction, Direction) {
    let mut in_loop: Vec<bool> = vec![false; map.len()];
    in_loop[start] = true;

    let first_direction = find_departure(map, start, line_size);
    let mut direction = first_direction;
    let mut position = direction.next_position(start, line_size);
    let mut tile = map.get(position).unwrap();
    let mut from = direction.reverse();

    in_loop[position] = true;

    while !matches!(tile, Tile::Start) {
        direction = tile.crawl(&from);
        from = direction.reverse();
        position = direction.next_position(position, line_size);
        tile = map.get(position).unwrap();
        in_loop[position] = true;
    }

    (in_loop, first_direction, direction)
}

fn get_start(map: &[Tile]) -> usize {
    let mut start: Option<usize> = None;
    for (position, tile) in map.iter().enumerate() {
        if matches!(tile, Tile::Start) {
            start = Some(position);
            break;
        }
    }
    start.unwrap()
}

fn count_within(map: &[Tile], line_size: usize) -> u32 {
    let mut inside = true;
    let mut total: u32 = 0;
    let mut last: Option<&Tile> = None;

    for (i, tile) in map.iter().enumerate() {
        if i % line_size == 0 {
            inside = false;
            last = None;
        }

        match (tile, last) {
            (Tile::Ground, _) => {
                if inside {
                    total += 1;
                }
            }
            (Tile::NS, _) => inside = !inside,
            (Tile::EW, _) => (),
            (Tile::NE | Tile::SE, _) => last = Some(tile),
            (Tile::NW, Some(Tile::NE)) => (),
            (Tile::NW, Some(Tile::SE)) => inside = !inside,
            (Tile::SW, Some(Tile::NE)) => inside = !inside,
            (Tile::SW, Some(Tile::SE)) => (),
            (_, _) => unreachable!("{tile:?} should not follow {last:?}"),
        }
    }

    total
}
