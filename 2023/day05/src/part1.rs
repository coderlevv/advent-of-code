pub fn solve(input: &[String]) {
    let mut seeds: Vec<u64> = vec![];
    let mut tags: Vec<String> = vec![];
    let mut maps: Vec<Vec<(u64, u64, u64)>> = vec![];
    let mut tmp: Vec<(u64, u64, u64)> = vec![];
    let mut line_iter = input.iter();
    if let Some(line) = line_iter.next() {
        let sep = line.chars().position(|c| c == ':').unwrap();
        seeds = line[sep+1..]
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
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
        let range: Vec<u64> = line
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        tmp.push((range[1], range[1]+range[2], range[0]));
    }
    if !tmp.is_empty() {
        maps.push(tmp);
    }

    let mut result: Vec<u64> = vec![];
    for seed in seeds {
        let mut res = seed;
        let mut from = "seed";
        loop {
            let map_idx = tags.iter().position(|t| t.starts_with(from)).unwrap();
            let map = &maps[map_idx];
            let to = tags[map_idx].split('-').nth(2).unwrap();
            for (dest_start, dest_end, src_start) in map {
                if res >= *dest_start && res <= *dest_end {
                    let delta = res - dest_start;
                    res = src_start + delta;
                    break;
                }
            }
            if to == "location" { break; }
            from = to;
        }
        result.push(res);
    }
    println!("{:?}", result.iter().min().unwrap());
}