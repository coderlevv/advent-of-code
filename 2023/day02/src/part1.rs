const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

use regex::Regex;

pub fn solve(input: &[String]) {
    let regex_game = Regex::new(r"Game (\d+)").unwrap();
    let regex_red = Regex::new(r"(\d+) red").unwrap();
    let regex_green = Regex::new(r"(\d+) green").unwrap();
    let regex_blue = Regex::new(r"(\d+) blue").unwrap();
    let mut possible: Vec<u32> = vec![];
    for line in input {
        let record_split: Vec<&str> = line.split(':').collect();
        let Some(capt) = regex_game.captures(record_split[0]) else { panic!("missing id") };
        let game_id = &capt[1].trim().parse::<u32>().unwrap();
        let game_split: Vec<&str> = record_split[1].split(';').collect();
        let mut is_possible = true;
        for game in game_split {
            if let Some(capt) = regex_red.captures(game) {
                let red = &capt[1].trim().parse::<u32>().unwrap();
                if *red > RED { is_possible = false; }
            }
            if let Some(capt) = regex_green.captures(game) {
                let green = &capt[1].trim().parse::<u32>().unwrap();
                if *green > GREEN { is_possible = false; }
            }
            if let Some(capt) = regex_blue.captures(game) {
                let blue = &capt[1].trim().parse::<u32>().unwrap();
                if *blue > BLUE { is_possible = false; }
            }
        }
        if is_possible { possible.push(*game_id); }
    }
    println!("{:?}", possible.iter().sum::<u32>());
}