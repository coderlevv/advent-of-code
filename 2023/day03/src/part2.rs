use std::collections::{HashMap, HashSet};

fn has_symbol(
    i: &usize,
    j: &usize,
    grid: &Vec<Vec<char>>,
    star_pos: &mut HashSet<(usize, usize)>
) -> (bool, bool) {
    let row_max = grid.len() - 1;
    let col_max = grid[0].len() - 1;
    if *i > 0 && grid[*i-1][*j] != '.' && !grid[*i-1][*j].is_ascii_digit() {
        if grid[*i-1][*j] == '*' {
            star_pos.insert((*i-1, *j));
            return (true, true);
        }
        return (true, false);
    }
    if *i < row_max && grid[*i+1][*j] != '.' && !grid[*i+1][*j].is_ascii_digit() {
        if grid[*i+1][*j] == '*' {
            star_pos.insert((*i+1, *j));
            return (true, true);
        }
        return (true, false);
    }
    if *j > 0 && grid[*i][*j-1] != '.' && !grid[*i][*j-1].is_ascii_digit() {
        if grid[*i][*j-1] == '*' {
            star_pos.insert((*i, *j-1));
            return (true, true);
        }
        return (true, false);
    }
    if *j < col_max && grid[*i][*j+1] != '.' && !grid[*i][*j+1].is_ascii_digit() {
        if grid[*i][*j+1] == '*' {
            star_pos.insert((*i, *j+1));
            return (true, true);
        }
        return (true, false);
    }
    if *i > 0 && *j > 0 && grid[*i-1][*j-1] != '.' && !grid[*i-1][*j-1].is_ascii_digit() {
        if grid[*i-1][*j-1] == '*' {
            star_pos.insert((*i-1, *j-1));
            return (true, true);
        }
        return (true, false);
    }
    if *i < row_max && *j < col_max && grid[*i+1][*j+1] != '.' && !grid[*i+1][*j+1].is_ascii_digit() {
        if grid[*i+1][*j+1] == '*' {
            star_pos.insert((*i+1, *j+1));
            return (true, true);
        }
        return (true, false);
    }
    if *i > 0 && *j < col_max && grid[*i-1][*j+1] != '.' && !grid[*i-1][*j+1].is_ascii_digit() {
        if grid[*i-1][*j+1] == '*' {
            star_pos.insert((*i-1, *j+1));
            return (true, true);
        }
        return (true, false);
    }
    if *i < row_max && *j > 0 && grid[*i+1][*j-1] != '.' && !grid[*i+1][*j-1].is_ascii_digit() {
        if grid[*i+1][*j-1] == '*' {
            star_pos.insert((*i+1, *j-1));
            return (true, true);
        }
        return (true, false);
    }
    (false, false)
}

pub fn solve(input: &Vec<Vec<char>>) {
    let mut number = String::new();
    let mut parse_number = false;
    let mut is_part_num = false;
    let mut has_gear_flag = false;
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut star_pos: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = &input[i][j];
            if c.is_ascii_digit() {
                number.push(*c);
                if !parse_number { parse_number = true; }
                let (part_flag, gear_flag) = has_symbol(&i, &j, input, &mut star_pos);
                if part_flag { is_part_num = true; }
                if gear_flag { has_gear_flag = true; }
            } else {
                if parse_number {
                    if is_part_num && has_gear_flag {
                        assert!(star_pos.len() == 1);
                        let key = star_pos.iter().next().unwrap();
                        gears.entry(*key).or_default().push(number.parse::<u32>().unwrap());
                        is_part_num = false;
                        has_gear_flag = false;
                    }
                    number.clear();
                    star_pos.clear();
                }
                parse_number = false;
            }
        }
        if parse_number {
            if is_part_num && has_gear_flag {
                assert!(star_pos.len() == 1);
                let key = star_pos.iter().next().unwrap();
                gears.entry(*key).or_default().push(number.parse::<u32>().unwrap());
                is_part_num = false;
                has_gear_flag = false;
            }
            number.clear();
            star_pos.clear();
        }
        parse_number = false;
    }
    let mut ratio: Vec<u32> = vec![];
    for num in gears.values() {
        // gear parts have exactly 2 associated numbers 
        if num.len() == 2 {
            ratio.push(num[0] * num[1]);
        }
    }
    println!("{}", ratio.iter().sum::<u32>());
}