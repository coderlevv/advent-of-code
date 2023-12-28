
fn has_symbol(i: &usize, j: &usize, grid: &Vec<Vec<char>>) -> bool {
    let row_max = grid.len() - 1;
    let col_max = grid[0].len() - 1;
    if *i > 0 && grid[*i-1][*j] != '.' && !grid[*i-1][*j].is_ascii_digit() {
        return true;
    }
    if *i < row_max && grid[*i+1][*j] != '.' && !grid[*i+1][*j].is_ascii_digit() {
        return true;
    }
    if *j > 0 && grid[*i][*j-1] != '.' && !grid[*i][*j-1].is_ascii_digit() {
        return true;
    }
    if *j < col_max && grid[*i][*j+1] != '.' && !grid[*i][*j+1].is_ascii_digit() {
        return true;
    }
    if *i > 0 && *j > 0 && grid[*i-1][*j-1] != '.' && !grid[*i-1][*j-1].is_ascii_digit() {
        return true;
    }
    if *i < row_max && *j < col_max && grid[*i+1][*j+1] != '.' && !grid[*i+1][*j+1].is_ascii_digit() {
        return true;
    }
    if *i > 0 && *j < col_max && grid[*i-1][*j+1] != '.' && !grid[*i-1][*j+1].is_ascii_digit() {
        return true;
    }
    if *i < row_max && *j > 0 && grid[*i+1][*j-1] != '.' && !grid[*i+1][*j-1].is_ascii_digit() {
        return true;
    }
    false
}

pub fn solve(input: &Vec<Vec<char>>) {
    let mut number = String::new();
    let mut parse_number = false;
    let mut part_num: Vec<u32> = vec![];
    let mut is_part_num = false;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = &input[i][j];
            if c.is_ascii_digit() {
                number.push(*c);
                if !parse_number { parse_number = true; }
                if !is_part_num { is_part_num = has_symbol(&i, &j, input); }
            } else {
                if parse_number {
                    if is_part_num {
                        part_num.push(number.parse::<u32>().unwrap());
                        is_part_num = false;
                    }
                    number.clear();
                }
                parse_number = false;
            }
        }
        if parse_number {
            if is_part_num {
                part_num.push(number.parse::<u32>().unwrap());
                is_part_num = false;
            }
            number.clear();
        }
        parse_number = false;
    }
    println!("{:?}", part_num.iter().sum::<u32>());
}