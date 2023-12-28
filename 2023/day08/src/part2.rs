use std::collections::HashMap;
use std::iter;
use num::integer;

pub fn solve(input: &[String]) {
    let mut nodes: HashMap<String,(String, String)> = HashMap::new();
    let mut instr: Vec<char> = vec![];
    let re = regex::Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
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
    let mut node_vec: Vec<String> = nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .cloned()
        .collect();
    let mut cycle: Vec<Option<u64>> = Vec::from_iter(iter::repeat(None).take(node_vec.len()));
    let mut steps: u64 = 0;
    let mut done = false;
    while !done {
        for dir in &instr {
            let mut nxt_vec: Vec<String> = vec![];
            for node in &node_vec {
                let nxt = nodes.get(node).unwrap();
                match dir {
                    'L' => nxt_vec.push(nxt.0.clone()),
                    'R' => nxt_vec.push(nxt.1.clone()),
                    _ => panic!("Unknown direction!")
                };
            }
            node_vec = nxt_vec;
            steps += 1;
        }
        for (idx, node) in node_vec.iter().enumerate() {
            if node.ends_with('Z') && cycle[idx].is_none() {
                cycle[idx] = Some(steps);
            }
        }
        done = cycle.iter().all(|c| c.is_some());
    }
    let mut lcm: u64 = integer::lcm(cycle[0].unwrap(), cycle[1].unwrap());
    let mut k: usize = 2;
    while k < cycle.len() {
        lcm = integer::lcm(lcm, cycle[k].unwrap());
        k += 1;
    }
    println!("{:?}", lcm);
}