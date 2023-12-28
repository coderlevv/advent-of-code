use crate::hand::{Hand, HandType};
use std::cmp::Ordering;

pub fn solve(input: &[String]) {
    let strength: Vec<char> = vec![
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'
    ];
    let mut hand_list: Vec<Hand> = vec![];
    for line in input {
        let split: Vec<_> = line
            .split_whitespace()
            .collect();
        let cards:Vec<_> = split[0].chars().collect();
        let bid = split[1].parse::<u64>().unwrap();
        let mut hand = Hand::new(cards, bid);
        hand.set_type();
        assert!(*hand.get_type() != HandType::Unknown);
        hand_list.push(hand);
    }
    hand_list.sort_by(|a, b| {
        let a_type = a.subst_type;
        let b_type = b.subst_type;
        if a.subst_type != b.subst_type {
            if a_type < b_type {
                return Ordering::Less;
            }
            if a_type > b_type {
                return Ordering::Greater;
            }
        } else {
            for i in 0..5 {
                if a.strength(i, &strength) < b.strength(i, &strength) {
                    return Ordering::Less;
                }
                if a.strength(i, &strength) > b.strength(i, &strength) {
                    return Ordering::Greater;
                }
            }
        }
        Ordering::Equal
    });
    let total = hand_list
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1) as u64)
        .sum::<u64>();
        
    println!("{:?}", total);
}