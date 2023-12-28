pub fn solve(input: &Vec<String>) {
    let time: Vec<u32> = input[0][5..]
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();

    let dist: Vec<u32> = input[1][9..]
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();

    let mut possible: Vec<u32> = vec![];
    for i in 0..time.len() {
        let race_time = time[i];
        let best_dist = dist[i];
        let mut count: u32 = 0;
        for charge_time in 1..race_time - 1 {
            if (race_time - charge_time) * charge_time > best_dist {
                count += 1;
            }
        }
        possible.push(count);
    }
    println!("{}", possible.iter().product::<u32>());
}