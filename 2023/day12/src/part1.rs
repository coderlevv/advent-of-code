use crate::arrange::count_arrangements;
//use crate::search::search; 

pub fn solve(input: &[String]) {
    //let mut counts: Vec<u64> = vec![];
    let mut count: u64 = 0;
    for line in input {
        let spl: Vec<_> = line
            .split_whitespace()
            .collect();
        let mut cond: Vec<char> = spl[0].chars().collect();
        let groups: Vec<_> = spl[1]
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        // let count = search(&mut cond, &groups);
        // counts.push(count);
        cond.push('.');
        count += count_arrangements(cond.clone(), groups.clone());
    }
    println!("{}", count);
}
