use std::fmt::Display;

use regex::Regex;

fn main() {
    assert_eq!(
        part_1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        ),
        8
    );

    assert_eq!(
        part_2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        ),
        2286
    );
}

fn part_1(input: &str) -> u32 {
    let mut total: u32 = 0;

    for line in input.lines() {
        let game_info = parse_game(line);
        if game_info.colors.possible(12, 13, 14) {
            total += game_info.id
        }
    }

    //println!("{total}");
    total
}

fn part_2(input: &str) -> u32 {
    let mut total: u32 = 0;

    for line in input.lines() {
        let game_info = parse_game(line);
        total += game_info.colors.power()
    }

    //println!("{total}");
    total
}

#[derive(Clone, Copy)]
struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

impl Colors {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn possible(self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        (self.red <= max_red) & (self.green <= max_green) & (self.blue <= max_blue)
    }

    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "R({}) G({}) B({})", self.red, self.green, self.blue)
    }
}

fn max_colors(colors1: Colors, colors2: Colors) -> Colors {
    Colors {
        red: std::cmp::max(colors1.red, colors2.red),
        green: std::cmp::max(colors1.green, colors2.green),
        blue: std::cmp::max(colors1.blue, colors2.blue),
    }
}

struct GameInfo {
    id: u32,
    colors: Colors,
}

impl GameInfo {
    fn new(id: u32) -> Self {
        Self {
            id,
            colors: Colors::new(),
        }
    }
}

impl Display for GameInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game {}: {}", self.id, self.colors)
    }
}

fn parse_game(game: &str) -> GameInfo {
    let first_digit = game.find(char::is_numeric).unwrap();
    let colon_id = game.find(':').unwrap();

    let game_id = game[first_digit..colon_id].parse::<u32>().unwrap();

    let mut game_info = GameInfo::new(game_id);

    for draw in game[colon_id..].split("; ") {
        let draw_colors = count_colors(draw);
        game_info.colors = max_colors(game_info.colors, draw_colors);
    }

    //println!("{result}");

    game_info
}

fn count_colors(draw: &str) -> Colors {
    let red: Regex = Regex::new(r"[0-9]+ red").unwrap();
    let green: Regex = Regex::new(r"[0-9]+ green").unwrap();
    let blue: Regex = Regex::new(r"[0-9]+ blue").unwrap();
    let mut colors = Colors::new();

    colors.red = match red.find(draw) {
        Some(mat) => mat.as_str().replace(" red", "").parse::<u32>().unwrap(),
        None => 0,
    };

    colors.green = match green.find(draw) {
        Some(mat) => mat.as_str().replace(" green", "").parse::<u32>().unwrap(),
        None => 0,
    };

    colors.blue = match blue.find(draw) {
        Some(mat) => mat.as_str().replace(" blue", "").parse::<u32>().unwrap(),
        None => 0,
    };

    colors
}
