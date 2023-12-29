use memoize::memoize;

#[memoize]
fn arrange(combs: Vec<char>, groups: Vec<i32>, open: bool) -> u64 {
    let current = combs[0];
    let group = groups[0];
    let end_of_spring: bool = combs.len() == 1;
    let end_of_group: bool = groups.len() == 1;
    
    if group >= 0 {
        match current {
            '#' => {
                let new_comb = &combs[1..];
                let mut new_groups: Vec<i32> = vec![group - 1];
                new_groups.extend(&groups[1..]);
                return arrange(new_comb.to_vec(), new_groups.clone(), true);
            },
            '.' => {
                if open && group != 0 {
                    return 0;
                }
                if end_of_spring {
                    if end_of_group && group == 0 { return 1; } else { return 0; }
                }
                if open {
                    let new_comb = &combs[1..];
                    let mut new_groups: Vec<i32> = vec![];
                    if end_of_group {
                        new_groups.push(0);
                    } else {
                        new_groups.extend(&groups[1..]);
                    }
                    return arrange(new_comb.to_vec(), new_groups.clone(), false);
                } else {
                    let new_comb = &combs[1..];
                    return arrange(new_comb.to_vec(), groups.clone(), false);
                }
            },
            '?' => {
                if open {
                    if group == 0 {
                    let new_comb = &combs[1..];
                    let mut new_groups: Vec<i32> = vec![];
                    if end_of_group {
                        new_groups.push(0);
                    } else {
                        new_groups.extend(&groups[1..]);
                    }
                    return arrange(new_comb.to_vec(), new_groups.clone(), false);
                    } else {
                        let new_comb = &combs[1..];
                        let mut new_groups: Vec<i32> = vec![group - 1];
                        new_groups.extend(&groups[1..]);
                        return arrange(new_comb.to_vec(), new_groups.clone(), true);
                    }
                } else {
                    let new_comb = &combs[1..];
                    let mut new_groups: Vec<i32> = vec![group - 1];
                    new_groups.extend(&groups[1..]);
                    let a = arrange(new_comb.to_vec(), new_groups.clone(), true);
                    let b = arrange(new_comb.to_vec(), groups.clone(), false);
                    return a + b;
                }
            },
            _ => panic!("Unknown symbol!")
        }
    }
    // if group < 0, return 0
    0
}

pub fn count_arrangements(springs: Vec<char>, groups: Vec<i32>) -> u64 {
    arrange(springs, groups, false)
}

// brute-force implementation to recursively check all possible combinations
// not a viable solution for part 2 as it takes too long to search 
// through the unfolded combinations.
// Thanks to https://github.com/blu3r4y/AdventOfCode2023/tree/main for the
// idea to solve part 2.
//
// pub fn search(seq: &mut [char], groups: &[u32]) -> u64 {
//     let mut count = 0;
//     if seq.iter().all(|c| *c != '?') {
//         let seq_groups = find_groups(seq);
//         if is_equal(&seq_groups, groups) {
//             return 1
//         }
//         0
//     }
//     else {
//         let k = seq.iter().position(|c| *c == '?').unwrap();
//         let groups_so_far = find_groups(&seq[..k]);
//         if groups_so_far.len() > 1 {
//             for i in 0..groups_so_far.len()-1 {
//                 if i == groups.len() { return 0 }
//                 if groups_so_far[i] != groups[i] { return 0 }
//             }
//         }
//         seq[k] = '.';
//         count += search(seq, groups);
//         seq[k] = '#';
//         count += search(seq, groups);
//         seq[k] = '?';
//         count
//     }
// }

// fn is_equal(a: &[u32], b: &[u32]) -> bool {
//     if a.len() == b.len() {
//         a.iter()
//             .enumerate()
//             .map(|(i, n)| *n == b[i])
//             .all(|t| t)
//     } else {
//         false
//     }
// }

// fn find_groups(seq: &[char]) -> Vec<u32> {
//     let mut groups: Vec<u32> = vec![];
//     let mut count: u32 = 0;
//     let mut flushed = false;
//     for c in seq {
//         if *c == '#' {
//             count += 1;
//             flushed = false;
//         } else {
//             if count > 0 && !flushed {
//                 groups.push(count);
//                 flushed = true;
//             }
//             count = 0;
//         }
//     }
//     if count > 0 {
//         groups.push(count);
//     }
//     groups
// } 

// #[test]
// fn is_equal_test() {
//     assert!(is_equal(&[1,2,3], &[1,2,3]));
//     assert!(!is_equal(&[1,2], &[1,2,3]));
//     assert!(!is_equal(&[1,3,2], &[1,2,3]));
//     assert!(!is_equal(&[1,2,3], &[1,2]));
//     assert!(!is_equal(&[1,2,3], &[2,1,3]));
// }

// #[test]
// fn count_groups_test() {
//     assert_eq!(find_groups(&"#.#".chars().collect::<Vec<char>>()), vec![1, 1]);
//     assert_eq!(find_groups(&"##...#".chars().collect::<Vec<char>>()), vec![2, 1]);
//     assert_eq!(find_groups(&"...#...#..".chars().collect::<Vec<char>>()), vec![1, 1]);
//     assert_eq!(find_groups(&"#..##..###".chars().collect::<Vec<char>>()), vec![1, 2, 3]);
//     assert_eq!(find_groups(&".#.##.#.".chars().collect::<Vec<char>>()), vec![1, 2, 1]);
//     assert_eq!(find_groups(&"...".chars().collect::<Vec<char>>()), vec![]);
//     assert_eq!(find_groups(&"###".chars().collect::<Vec<char>>()), vec![3]);
// }