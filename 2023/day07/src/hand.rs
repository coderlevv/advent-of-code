use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    Unknown,
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind  
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<char>,
    pub hand_type: HandType,
    pub subst_type: HandType,
    pub bid: u64
}

impl Hand {
    pub fn new(cards: Vec<char>, bid: u64) -> Self {
        Self {
            cards,
            hand_type: HandType::Unknown,
            subst_type: HandType::Unknown,
            bid
        }
    }

    fn determine_type(&self, cards: &[char]) -> HandType {
        let mut counts: HashMap<char, u32> = HashMap::new();
        for card in cards {
            *counts.entry(*card).or_default() += 1;
        }
        match counts.len() {
            1 => HandType::FiveKind,
            2 => {
                if counts.iter().any(|item| *item.1 == 4) {
                    HandType::FourKind
                } else {
                    HandType::FullHouse
                }
            },
            3 => {
                if counts.iter().any(|item| *item.1 == 3) {
                    HandType::ThreeKind
                } else {
                    HandType::TwoPair
                }
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Unknown hand type!")
        }
    }

    pub fn subst(
        &mut self,
        cards: &mut Vec<char>,
        possible: &HashSet<char>,
        best: &mut HandType
    ) {
        if let Some(pos) = cards.iter().copied().position(|c| c == 'J') {
            for subst in possible {
                cards[pos] = *subst;
                self.subst(cards, possible, best);
            }
        } else {
            let subst_type = self.determine_type(cards);
            if (subst_type as usize) > (*best as usize) {
                *best = subst_type;
            } 
        }
    }

    pub fn set_type(&mut self) {
        let cards = &self.cards;
        self.hand_type = self.determine_type(cards);
        self.subst_type = self.hand_type;

        if self.cards.contains(&'J') {
            let possible: HashSet<char> = HashSet::from_iter(
                cards.iter()
                .copied()
                .filter(|c| *c != 'J')
            );
            let mut subst_type = self.hand_type;
            self.subst(&mut cards.clone(), &possible, &mut subst_type);
            self.subst_type = subst_type;
        }
    }

    pub fn get_type(&self) -> &HandType {
        &self.hand_type
    }

    pub fn strength(&self, idx: usize, strength: &[char]) -> usize {
        strength.iter().position(|c| *c == self.cards[idx]).unwrap()
    }
}