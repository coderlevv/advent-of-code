use std::collections::HashMap;

pub fn solve(input: &Vec<String>) {
    let mut count: HashMap<u32, u32> = HashMap::new();
    let mut card_num: u32 = 0;
    let input_len = input.len() as u32;
    for line in input {
        card_num += 1;
        let rounds = *count.entry(card_num).or_insert(1);
        let sep = line.chars().position(|c| c == ':').unwrap();
        let spl: Vec<&str> = line[sep+1..].split('|').collect();
        let win: Vec<u32> = spl[0]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let num: Vec<u32> = spl[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let found = win.iter().filter(|n| num.contains(n)).count() as u32;
        for i in 0..found {
            for _ in 0..rounds {
                if card_num + i < input_len {
                    *count.entry(card_num + i + 1).or_insert(1) += 1;
                }
            }
        }
    }
    println!("{}", count.values().sum::<u32>());
}