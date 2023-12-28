use crate::search::{search, search_thread}; 
use crate::thread::Thread;
use std::collections::HashSet;

pub fn solve(input: &[String]) {
    let mut counts: Vec<u64> = vec![];
    for (i, line) in input.iter().enumerate() {
        println!("{} of {}", i, input.len());
        let spl: Vec<_> = line
            .split_whitespace()
            .collect();
        let cond: Vec<char> = spl[0].chars().collect();
        let mut xcond = cond.clone();
        let groups: Vec<_> = spl[1]
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        //println!("{:?}", groups);
        let mut xgroups = groups.clone();
        for _ in 0..4 {
            xcond.push('?');
            xcond.extend(cond.iter());
            xgroups.extend(groups.iter());
        }
        let mut thread = Thread::from_vec(xgroups, xcond);
        //let mut count: u64 = 0;
        let mut configs: HashSet<String> = HashSet::new();
        //println!("{}", spl[0]);
        search_thread(&mut thread, 0, &mut configs);
        counts.push(configs.len() as u64);
        //println!("{:?}: len={}: count={}", thread, thread.len(), count);
    }
    println!("{:?}", counts.iter().sum::<u64>());

}