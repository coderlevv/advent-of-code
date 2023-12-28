use regex::Regex;

pub fn solve(input: &[String]) {
    let regex_red = Regex::new(r"(\d+) red").unwrap();
    let regex_green = Regex::new(r"(\d+) green").unwrap();
    let regex_blue = Regex::new(r"(\d+) blue").unwrap();
    let mut power: Vec<u32> = vec![];
    for line in input {
        let record_split: Vec<&str> = line.split(':').collect();
        let game_split: Vec<&str> = record_split[1].split(';').collect();
        let mut red_max: u32 = 1;
        let mut green_max: u32 = 1;
        let mut blue_max: u32 = 1;
        for game in game_split {
            if let Some(capt) = regex_red.captures(game) {
                let red = &capt[1].trim().parse::<u32>().unwrap();
                if *red > red_max { red_max = *red; }
            }
            if let Some(capt) = regex_green.captures(game) {
                let green = &capt[1].trim().parse::<u32>().unwrap();
                if *green > green_max { green_max = *green; }
            }
            if let Some(capt) = regex_blue.captures(game) {
                let blue = &capt[1].trim().parse::<u32>().unwrap();
                if *blue > blue_max { blue_max = *blue; }
            }
        }
        power.push(red_max * green_max * blue_max);
    }
    println!("{:?}", power.iter().sum::<u32>());
}