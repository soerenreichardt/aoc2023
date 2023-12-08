use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

const CARDS: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

#[derive(Debug, Eq, PartialEq, Hash)]
struct Hand {
    cards: [char; 5]
}

#[repr(u8)]
#[derive(Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

pub fn score_hands(input: &str) -> u32 {
    let hands_with_bids = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| (Hand::from_str(hand).unwrap(), bid.parse::<u32>().unwrap()))
        .collect::<HashMap<Hand, u32>>();

    let mut hands = hands_with_bids.keys().collect::<Vec<_>>();
    hands.sort();
    hands.iter().enumerate().map(|(pos, hand)| {
        let bid = hands_with_bids.get(hand).unwrap();
        (pos + 1) as u32 * bid
    }).sum()
}

impl FromStr for Hand {
    type Err = Vec<char>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [char;5] = s.chars().collect::<Vec<_>>().try_into()?;
        Ok(Hand { cards })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type().partial_cmp(&other.hand_type()) {
            // Hand types are equal so decide ordering based on first card that differs
            None => Some(self.cards.iter().zip(other.cards.iter()).find_map(|(lhs, rhs)| {
                let lhs_pos = CARDS.iter().position(|&elem| elem.eq(lhs)).unwrap();
                let rhs_pos = CARDS.iter().position(|&elem| elem.eq(rhs)).unwrap();
                let ord = lhs_pos.cmp(&rhs_pos);
                if ord.is_ne() {
                    return Some(ord)
                }
                None
            }).unwrap()),
            ordering => ordering
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.clone() as u8).partial_cmp(&(other.clone() as u8)) {
            // We cannot decide equality based on the hand type only
            Some(Ordering::Equal) => None,
            other => other
        }
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Hand
{
    fn occurrences(cards: [char;5]) -> HashMap<char, u32> {
        let mut occurrences = HashMap::new();
        for card in cards {
            let entry = occurrences.entry(card).or_insert(0);
            *entry += 1;
        }
        occurrences
    }

    fn hand_type(&self) -> HandType {
        if self.cards.contains(&'J') {
            let mut max = None;
            for cards in self.replacements_for_joker() {
                if max.is_none() {
                    max = Some(cards);
                } else {
                    max = Some(max.unwrap().max(cards));
                }
            }
            return Self::hand_type_for(&max.unwrap());
        }
        Self::hand_type_for(self)
    }

    fn hand_type_for(hand: &Hand) -> HandType {
        let occurrences = Self::occurrences(hand.cards);
        let mut values = occurrences.values().filter(|value| **value != 0).collect::<Vec<_>>();
        values.sort_by(|a, b| b.cmp(a));
        match values[..] {
            [5, ..] => HandType::Five,
            [4, ..] => HandType::Four,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::Three,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            [..] => HandType::HighCard,
        }
    }

    fn replacements_for_joker(&self) -> Vec<Hand> {
        let mut replacements = Vec::new();
        for replacement in &CARDS[1..] {
            let mut new_cards = self.cards;
            for c in new_cards.iter_mut() {
                if *c == 'J' {
                    *c = *replacement
                }
            }
            replacements.push(Hand{ cards: new_cards });
        }
        replacements
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::day7::camel_cards::{Hand, HandType, score_hands};

    #[test]
    fn should_score_hands() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(score_hands(input), 5905);
    }

    #[test]
    fn should_sort_hand_types() {
        assert!(HandType::Five > HandType::Four);
        assert!(HandType::Four > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::Three);
        assert!(HandType::Three > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn should_order_tie() {
        let lhs = Hand { cards: ['Q', 'Q', 'Q', 'J', 'A'] };
        let rhs = Hand { cards: ['T', '5', '5', 'J', '5'] };
        let ordering = lhs.cmp(&rhs);
        assert_eq!(ordering, Ordering::Greater);

        let lhs = Hand { cards: ['K', 'K', '6', '7', '7'] };
        let rhs = Hand { cards: ['K', 'T', 'J', 'J', 'T'] };
        let ordering = lhs.cmp(&rhs);
        assert_eq!(ordering, Ordering::Greater);
    }

    #[test]
    fn should_compute_max() {
        let lhs = Hand { cards: ['T', '5', '5', '5', '5'] };
        let rhs = Hand { cards: ['T', '5', '5', '6', '5'] };

        assert_eq!(Hand { cards: ['T', '5', '5', '5', '5'] }, rhs.max(lhs));
    }
}