use std::collections::HashMap;

pub fn solve(input: &[String]) {
    let mut nodes: HashMap<String,(String, String)> = HashMap::new();
    let mut instr: Vec<char> = vec![];
    let re = regex::Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    //let mut k = 0;
    for (k, line) in input.iter().enumerate() {
        if k == 0 {
            instr = line.chars().collect();
        }
        if k > 1 {
            if let Some(dir) = re.captures(line) {
                let key = dir[1].to_string();
                let val = (dir[2].to_string(), dir[3].to_string());
                nodes.insert(key, val);
            }
        }
    }
    let mut node = "AAA".to_string();
    let mut steps: u32 = 0;
    while node != "ZZZ" {
        for dir in &instr {
            let nxt = nodes.get(&node).unwrap();
            node = match dir {
                'L' => nxt.0.clone(),
                'R' => nxt.1.clone(),
                _ => panic!("Unknown direction!")
            };
            steps += 1;
        }
    }
    println!("{}", steps);
}