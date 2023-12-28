pub fn solve(input: &[String]) {
    let mut seeds: Vec<(i64, i64)> = vec![];
    let mut tags: Vec<String> = vec![];
    let mut maps: Vec<Vec<(i64, i64, i64, i64)>> = vec![];
    let mut tmp: Vec<(i64, i64, i64, i64)> = vec![];
    let mut line_iter = input.iter();
    if let Some(line) = line_iter.next() {
        let sep = line.chars().position(|c| c == ':').unwrap();
        let seeds_tmp: Vec<i64> = line[sep+1..]
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        let mut k = 0;
        while k < seeds_tmp.len() {
            seeds.push((seeds_tmp[k], seeds_tmp[k]+seeds_tmp[k+1] - 1));
            k += 2;
        }
    }
    line_iter.next(); // skip first empty line
    for line in line_iter {
        if line.is_empty() {
            maps.push(tmp.clone());
            tmp.clear();
            continue;
        }
        if line.ends_with(':') {
            let sep = line.chars().position(|c| c == ' ').unwrap();
            tags.push(line[..sep].to_string());
            continue;
        }
        let range: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        tmp.push((range[1], range[1]+range[2]-1, range[0], range[0]+range[2]-1));
    }
    if !tmp.is_empty() {
        maps.push(tmp);
    }

    let mut result: Vec<i64> = vec![];
    for seed in seeds {
        let mut acc = vec![seed];
        for map in &maps {
            let mut k: usize = 0;
            let mut new_acc: Vec<(i64, i64)> = vec![];
            loop {
                let item = acc[k];
                for range in map {
                    if let Some(overlap) = overlap(item, (range.0, range.1)) {
                        let delta: i64 = range.2 - range.0; 
                        new_acc.push((overlap.0+delta, overlap.1+delta));
                    }
                }
                k += 1;
                if k == acc.len() { break; }
            }
            if !new_acc.is_empty() {
                acc = new_acc;
            }
        }
        if let Some(range_min) = acc.iter().map(|r| r.0).min() {
            result.push(range_min);
        }
    }
    println!("{:?}", result.iter().min().unwrap());
}

fn overlap(a: (i64, i64), b: (i64, i64)) -> Option<(i64, i64)> {
    if a.1 < b.0 || a.0 > b.1 {
         None
    } else if a.0 <= b.0 && b.1 <= a.1 {
            return Some((b.0, b.1));
        }
        else if a.0 <= b.0 && b.1 >= a.1 {
            return Some((b.0, a.1));
        }
        else if a.0 >= b.0 && b.1 <= a.1 {
            return Some((a.0, b.1));
        }
        else {
            return Some((a.0, a.1));
        }
} 

#[test]
fn overlap_test() {
    assert_eq!(overlap((2,8), (1,7)), Some((2,7)));
    assert_eq!(overlap((2,8), (1,8)), Some((2,8)));
    assert_eq!(overlap((2,8), (1,9)), Some((2,8)));
    assert_eq!(overlap((2,8), (3,5)), Some((3,5)));
    assert_eq!(overlap((2,8), (1,9)), Some((2,8)));
    assert_eq!(overlap((1,7), (2,8)), Some((2,7)));
    assert_eq!(overlap((1,8), (2,8)), Some((2,8)));
    assert_eq!(overlap((1,9), (2,8)), Some((2,8)));
    assert_eq!(overlap((3,5), (2,8)), Some((3,5)));
    assert_eq!(overlap((1,9), (2,8)), Some((2,8)));
}