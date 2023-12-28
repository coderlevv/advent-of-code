fn all_zero(x: &[i64]) -> bool {
    x.iter().all(|n| *n == 0)
}

pub fn extrapolate(x: &[i64]) -> i64 {
    if all_zero(x) {
        return 0;
    }
    let xprime: Vec<i64> = x
        .iter().skip(1)
        .zip(x.iter())
        .map(|(a, b)| a - b)
        .collect();
    x[x.len()-1] + extrapolate(&xprime)
}