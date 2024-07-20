// A game fails if one of the "cube critera" fails
// For each game need to figure out the game ID, if it passes add ID to sum
// For each game build a list of each round in the game
// For each round pull out how many of each color
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;


#[derive(Debug)]
struct GameResult {
    greens: u64,
    reds: u64,
    blues: u64
}

impl GameResult {
    fn power(&self) -> u64 {
        self.reds * self.greens * self.blues
    }
}


fn eval_game(round_s: &str) -> GameResult {
    // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let colors: [&str; 3] = ["red", "green", "blue"];
    let round_splits = round_s.split(";");
    let mut color_map = HashMap::new();
    for split in round_splits {
        let color_split = split.split(",");
        for color_s in color_split {
            let color_s = color_s.replace(" ", "");
            for color in colors {
                let result = color_s.replace(color, "");
                if let Ok(num) = result.parse::<u64>() {
                    match color_map.get_mut(color) {
                        Some(current_amount) => {
                            if *current_amount < num {
                                *current_amount = num;
                            }
                        }
                        None => {
                            color_map.insert(color, num);
                        }
                    }
                }
            }
        }
    }

    let result = GameResult {
        reds: *color_map.get("red").unwrap_or(&0),
        greens: *color_map.get("green").unwrap_or(&0),
        blues: *color_map.get("blue").unwrap_or(&0),
    };

    result
}


fn check_game(game: String) -> (u64, bool, u64) {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut passed = true;

    let game_split: Vec<&str> = game.split(":").collect();
    let id_str = game_split[0].replace("Game ", "").replace(":", "");
    let rounds = game_split[1];

    let result: GameResult = eval_game(rounds);

    if result.reds > max_red || result.greens > max_green || result.blues > max_blue {
        passed = false;
    }
    
    (id_str.parse::<u64>().unwrap(), passed, result.power())
    
}


fn get_input() -> Result<BufReader<File>, io::Error> { 
    let file = File::open("./part2input")?;
    let reader = BufReader::new(file);

    Ok(reader)
}


fn main() {
    let games = get_input();
    let mut sum = 0;
    let mut power = 0;
    for game in games.unwrap().lines() {
        let (id, passed, _power) = check_game(game.unwrap());
        if passed {
            sum += id;
        }
        power += _power;
    }

    println!("{}", sum);
    println!("{}", power);
}