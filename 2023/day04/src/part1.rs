pub fn solve(input: &Vec<String>) {
    let mut total: u32 = 0;
    for line in input {
        let colon = line.chars().position(|c| c == ':').unwrap();
        let spl: Vec<&str> = line[colon+1..].split('|').collect();
        let win: Vec<u32> = spl[0]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let num: Vec<u32> = spl[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let found = win.iter().filter(|n| num.contains(n)).count() as u32;
        if found > 0 {
            total += 2u32.pow(found - 1);
        }
    }
    println!("{}", total);
}