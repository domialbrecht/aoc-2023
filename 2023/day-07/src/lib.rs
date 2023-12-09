use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

impl HandType {
    // 55555 1
    // 55551 2
    // 55533 2
    // 55521 3
    // 55331 3
    // 11432 4
    // 12345 5
    // JJJJJ
    fn get_type(mut card_counts: HashMap<char, usize>) -> HandType {
        if let Some(i) = card_counts.get(&'J') {
            if card_counts.len() > 1 {
                let jcount = *i;
                card_counts.remove_entry(&'J');
                let mut best_key = 'J';
                card_counts
                    .iter()
                    .max_set_by_key(|&(_, value)| value)
                    .iter()
                    .for_each(|(char, _)| {
                        if card_score(**char) > card_score(best_key) {
                            best_key = **char
                        }
                    });
                card_counts.entry(best_key).and_modify(|e| *e += jcount);
            }
        }

        dbg!(&card_counts);

        if card_counts.len() == 1 {
            HandType::FiveKind
        } else if card_counts.len() == 2 && card_counts.values().any(|x| *x == 4) {
            HandType::FourKind
        } else if card_counts.len() == 2 {
            //33322
            //333JJ
            //JJJ22
            HandType::FullHouse
        } else if card_counts.len() == 3 && card_counts.values().any(|x| *x == 3) {
            HandType::ThreeKind
        } else if card_counts.len() == 3 {
            HandType::TwoPair
        } else if card_counts.len() == 4 {
            HandType::Pair
        } else if card_counts.len() == 5 {
            HandType::HighCard
        } else {
            panic!("Invalid hand type")
        }
    }
}

pub fn card_score(char: char) -> u32 {
    match char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        value => value.to_digit(10).unwrap(),
    }
}

pub type HandScore = (HandType, (u32, u32, u32, u32, u32));

pub struct Hand<'a> {
    cards: &'a str,
    handtype: HandType,
}

impl<'a> Hand<'a> {
    pub fn create(cards: &str) -> Hand {
        if cards.len() != 5 {
            panic!("Invalid hand")
        }
        Hand {
            cards,
            handtype: HandType::get_type(cards.chars().counts_by(|char| char)),
        }
    }

    pub fn hand_score(&self) -> HandScore {
        let card_score = self.cards.chars().map(card_score).collect_tuple().unwrap();
        (self.handtype, card_score)
    }
}
