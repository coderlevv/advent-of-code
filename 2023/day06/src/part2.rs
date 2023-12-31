pub fn solve(input: &[String]) {
    let race_time: u64 = input[0][5..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .fold(String::new(), |a, b| format!("{}{}", a, b))
        .parse::<u64>()
        .unwrap();

    let best_dist: u64 = input[1][9..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .fold(String::new(), |a, b| format!("{}{}", a, b))
        .parse::<u64>()
        .unwrap();

    let mut possible: Vec<u64> = vec![];
    let mut count: u64 = 0;
    for charge_time in 1..race_time - 1 {
        if (race_time - charge_time) * charge_time > best_dist {
            count += 1;
        }
    }
    possible.push(count);
    println!("{}", possible.iter().product::<u64>());
}