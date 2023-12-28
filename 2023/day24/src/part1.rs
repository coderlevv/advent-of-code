use crate::stone::Stone;

const TEST_AREA: (f64, f64) = (200000000000000.0, 400000000000000.0);

pub fn solve(input: &[String]) {
    let mut stones: Vec<Stone> = vec![];
    for line in input {
        let stone = Stone::from_str(line);
        stones.push(stone);
    }
    
    let mut count: u32 = 0;
    for i in 1..stones.len() {
        let (a, b) = stones.split_at(i);
        let stone = &a[a.len()-1];
        for other in b {
            if let Some(inter) = stone.intersection_2d(other) {
                if !stone.in_the_past(&inter.0) && !other.in_the_past(&inter.0)
                    && (inter.0 >= TEST_AREA.0 && inter.0 <= TEST_AREA.1) 
                    && (inter.1 >= TEST_AREA.0 && inter.1 <= TEST_AREA.1) {
                        count += 1;
                }
            }
        }
    }
    println!("{:?}", count);
}