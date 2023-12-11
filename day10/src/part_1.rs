use std::collections::HashMap;

use crate::{Direction, Tile};

pub(crate) fn part_1(input: &str) -> u32 {
    let line_size = input.lines().next().unwrap().chars().count();

    let (start, map) = parse_input(input);

    let mut direction = find_departure(&map, start, line_size);
    let mut position = direction.next_position(start, line_size);
    let mut tile = map.get(&position).unwrap();
    let mut from = direction.reverse();

    let mut length: u32 = 1;

    while !matches!(tile, Tile::Start) {
        length += 1;
        direction = tile.crawl(&from);
        from = direction.reverse();
        position = direction.next_position(position, line_size);
        tile = map.get(&position).unwrap();
    }

    println!("{}", length / 2);
    length / 2
}

fn find_departure(map: &HashMap<usize, Tile>, start: usize, line_size: usize) -> Direction {
    if start > line_size {
        if let Some(Tile::NS | Tile::SE | Tile::SW) = map.get(&(start - line_size)) {
            return Direction::North;
        }
    }

    if start % line_size != 0 {
        if let Some(Tile::NE | Tile::SE | Tile::EW) = map.get(&(start - 1)) {
            return Direction::West;
        }
    }

    if start % line_size != line_size {
        if let Some(Tile::NW | Tile::SW | Tile::EW) = map.get(&(start + 1)) {
            return Direction::East;
        }
    }

    if let Some(Tile::NS | Tile::SE | Tile::SW) = map.get(&(start + line_size)) {
        return Direction::South;
    }

    panic!("Cannot find a direction to start")
}

fn parse_input(input: &str) -> (usize, HashMap<usize, Tile>) {
    let mut tiles: HashMap<usize, Tile> = HashMap::new();
    let mut position: usize = 0;
    let mut start_position: Option<usize> = None;

    for c in input.chars() {
        if (c == ' ') | (c == '\t') | (c == '\n') {
            continue;
        }

        position += 1;

        if c == '.' {
            continue;
        }

        let tile: Tile = match c {
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            'S' => {
                start_position = Some(position - 1);
                Tile::Start
            }
            _ => unreachable!("Unepexted character: {c}"),
        };

        tiles.insert(position - 1, tile);
    }

    (start_position.unwrap(), tiles)
}
