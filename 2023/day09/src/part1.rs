pub fn solve(input: &[String]) {
    let mut extra: Vec<i32> = vec![];
    for line in input {
        let x: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        extra.push(extrapolate(&x));
    }
    println!("{:?}", extra.iter().sum::<i32>());
}

fn all_zero(x: &[i32]) -> bool {
    x.iter().all(|n| *n == 0)
}

fn extrapolate(x: &[i32]) -> i32 {
    if all_zero(x) {
        return 0;
    }
    let xprime: Vec<i32> = x
        .iter().skip(1)
        .zip(x.iter())
        .map(|(a, b)| a - b)
        .collect();
    x[x.len()-1] + extrapolate(&xprime)
}