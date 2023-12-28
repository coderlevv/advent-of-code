use std::collections::HashMap;

fn collect_digits(line: &str, conv: &HashMap<String, u32>) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    for (i, c) in line.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            digits.push(d); 
        } else {
            for (from, to) in conv {
                if line[i..].starts_with(from) {
                    digits.push(*to);
                    break;
                }
            }
        }
    }
    digits
}

pub fn solve(input: &[String]) {
    let conv: HashMap<String, u32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);
    let mut vals: Vec<u32> = vec![];
    for line in input {
        let digits = collect_digits(line, &conv);
        let code = format!("{}{}", digits[0], digits[digits.len()-1]);
        vals.push(code.parse::<u32>().unwrap());
    }
    println!("{}", vals.iter().sum::<u32>());
}