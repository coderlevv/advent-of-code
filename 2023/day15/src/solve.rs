use regex::Regex;
use std::collections::HashMap;
type Lens = (String, u32);

pub fn solve(input: &[String]) {
    let lines: Vec<_> = input[0].split(',').collect();
    let mut hash_sum: u32 = 0;
    for line in &lines {
        hash_sum += hash(line);
    }
    // solution part 1
    println!("{:?}", hash_sum);

    let re = Regex::new(r"(\w+)([-=])(\d+)?").unwrap();
    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
    for line in &lines {
        let capt = re.captures(line);
        if let Some(capt) = capt {
            let label = capt[1].to_string();
            let key = hash(&capt[1]);
            let lenses = boxes.entry(key).or_default();
            match &capt[2] {
                "=" => {
                    let val = capt[3].parse::<u32>().unwrap();
                    if let Some(idx) = get_index(&capt[1], lenses) {
                        lenses[idx] = (label, val);
                    } else {
                        lenses.push((label, val));
                    }
                },
                "-" => {
                    if let Some(idx) = get_index(&capt[1], lenses) {
                        lenses.remove(idx);
                    }
                },
                _ => panic!("Unknown symbol!")
            }
        }
    }

    let mut power: Vec<u32> = vec![];
    for (key, lenses) in boxes {
        for (idx, lens) in lenses.iter().enumerate() {
            power.push(
                (key + 1) * ((idx + 1) as u32) * lens.1
            );
        }
    }
    println!("{:?}", power.iter().sum::<u32>());
}

fn hash (s: &str) -> u32 {
    let mut current: u32 = 0;
    for c in s.chars() {
        current += c as u32;
        current = (current * 17) % 256; 
    }
    current
}

fn get_index(label: &str, lenses: &[Lens]) -> Option<usize> {
    for (idx, lens) in lenses.iter().enumerate() {
        if lens.0 == label {
            return Some(idx)
        }
    }
    None
}
