use crate::search::count_arrangements;

pub fn solve(input: &[String]) {
    let mut count: u64 = 0;
    for line in input {
        let spl: Vec<_> = line
            .split_whitespace()
            .collect();
        let cond: Vec<char> = spl[0].chars().collect();
        let mut xcond = cond.clone();
        let groups: Vec<_> = spl[1]
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut xgroups = groups.clone();
        for _ in 0..4 {
            xcond.push('?');
            xcond.extend(cond.iter());
            xgroups.extend(groups.iter());
        }
        xcond.push('.');
        count += count_arrangements(xcond.clone(), xgroups.clone());
    }
    println!("{}", count);
}