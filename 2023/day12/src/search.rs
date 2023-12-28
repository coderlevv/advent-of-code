use crate::thread::{Thread, BlockType};
use std::collections::HashSet;

pub fn search_thread(thread: &mut Thread, pos: usize, configs: &mut HashSet<String>) {
    if 2 * pos >= thread.seq.len() {
        return;
    }
    //display(thread);
    if thread.is_consistent() {
        //*count += 1;
        //print!("consitent: {}", thread.output());
        configs.insert(thread.output());
    }
    // can gaps still be expanded?
    if thread.len() <= thread.pat.len() {
        //let mut k = 0;
        while thread.len() <= thread.pat.len() {
            search_thread(&mut thread.clone(), pos+1, configs);
            thread.seq[2 * pos].n += 1;
        }
    }
}

pub fn display(thread: &Thread) {
    for b in &thread.seq {
        for _ in 0..b.n {
            match b.block_type {
                BlockType::Block => print!("#"),
                BlockType::Gap => print!(".")
            }
        }
    }
    println!();
}


pub fn search(seq: &mut [char], groups: &[u32]) -> u64 {
    let mut count = 0;
    if seq.iter().all(|c| *c != '?') {
        let seq_groups = find_groups(seq);
        if is_equal(&seq_groups, groups) {
            return 1
        }
        0
    }
    else {
        let k = seq.iter().position(|c| *c == '?').unwrap();
        let groups_so_far = find_groups(&seq[..k]);
        if groups_so_far.len() > 1 {
            for i in 0..groups_so_far.len()-1 {
                if i == groups.len() { return 0 }
                if groups_so_far[i] != groups[i] { return 0 }
            }
        }
        seq[k] = '.';
        count += search(seq, groups);
        seq[k] = '#';
        count += search(seq, groups);
        seq[k] = '?';
        count
    }
}

fn is_equal(a: &[u32], b: &[u32]) -> bool {
    if a.len() == b.len() {
        a.iter()
            .enumerate()
            .map(|(i, n)| *n == b[i])
            .all(|t| t)
    } else {
        false
    }
}

fn find_groups(seq: &[char]) -> Vec<u32> {
    let mut groups: Vec<u32> = vec![];
    let mut count: u32 = 0;
    let mut flushed = false;
    for c in seq {
        if *c == '#' {
            count += 1;
            flushed = false;
        } else {
            if count > 0 && !flushed {
                groups.push(count);
                flushed = true;
            }
            count = 0;
        }
    }
    if count > 0 {
        groups.push(count);
    }
    groups
} 

#[test]
fn is_equal_test() {
    assert!(is_equal(&[1,2,3], &[1,2,3]));
    assert!(!is_equal(&[1,2], &[1,2,3]));
    assert!(!is_equal(&[1,3,2], &[1,2,3]));
    assert!(!is_equal(&[1,2,3], &[1,2]));
    assert!(!is_equal(&[1,2,3], &[2,1,3]));
}

#[test]
fn count_groups_test() {
    assert_eq!(find_groups(&"#.#".chars().collect::<Vec<char>>()), vec![1, 1]);
    assert_eq!(find_groups(&"##...#".chars().collect::<Vec<char>>()), vec![2, 1]);
    assert_eq!(find_groups(&"...#...#..".chars().collect::<Vec<char>>()), vec![1, 1]);
    assert_eq!(find_groups(&"#..##..###".chars().collect::<Vec<char>>()), vec![1, 2, 3]);
    assert_eq!(find_groups(&".#.##.#.".chars().collect::<Vec<char>>()), vec![1, 2, 1]);
    assert_eq!(find_groups(&"...".chars().collect::<Vec<char>>()), vec![]);
    assert_eq!(find_groups(&"###".chars().collect::<Vec<char>>()), vec![3]);
}