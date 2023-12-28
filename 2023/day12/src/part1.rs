use crate::search::search; 

pub fn solve(input: &[String]) {
    let mut counts: Vec<u64> = vec![];
    for line in input {
        let spl: Vec<_> = line
            .split_whitespace()
            .collect();
        let mut cond: Vec<char> = spl[0].chars().collect();
        let groups: Vec<_> = spl[1]
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let count = search(&mut cond, &groups);
        counts.push(count);
    }
    println!("{}", counts.iter().sum::<u64>());
}
