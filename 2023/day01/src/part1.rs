pub fn solve(input: &[String]) {
    let mut vals: Vec<u32> = vec![];
    for line in input {
        let mut digits: Vec<u32> = vec![];
        for c in line.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d); 
            }
        }
        if !digits.is_empty() {
            let dd = format!("{}{}", digits[0], digits[digits.len()-1]);
            vals.push(dd.parse::<u32>().unwrap());
        }
    }
    println!("{}", vals.iter().sum::<u32>());
}