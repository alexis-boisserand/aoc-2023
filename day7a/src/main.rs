#![feature(slice_group_by)]

use core::cmp::PartialEq;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        use Card::*;
        match value {
            '2' => _2,
            '3' => _3,
            '4' => _4,
            '5' => _5,
            '6' => _6,
            '7' => _7,
            '8' => _8,
            '9' => _9,
            'T' => T,
            'J' => J,
            'Q' => Q,
            'K' => K,
            'A' => A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    type_: HandType,
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        let mut sorted = cards.clone();
        sorted.sort();
        let mut groups: Vec<&[Card]> = sorted.group_by(PartialEq::eq).collect();
        groups.sort_by(|a, b| b.len().cmp(&a.len()));
        let type_ = match groups[..] {
            [[_, _, _, _, _]] => HandType::FiveOfAKind,
            [[_, _, _, _], _] => HandType::FourOfAKind,
            [[_, _, _], [_, _]] => HandType::FullHouse,
            [[_, _, _], [_], [_]] => HandType::ThreeOfAKind,
            [[_, _], [_, _], [_]] => HandType::TwoPairs,
            [[_, _], [_], [_], [_]] => HandType::OnePair,
            _ => HandType::HighCard,
        };
        Self { cards, type_ }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let eq = self.type_.cmp(&other.type_);
        eq.is_eq()
            .then_some(self.cards.cmp(&other.cards))
            .or(Some(eq))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut hands: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|l| {
            let (card, bid) = l.split_once(' ').unwrap();
            (
                Hand::new(
                    card.chars()
                        .map(|c| c.into())
                        .collect::<Vec<Card>>()
                        .try_into()
                        .unwrap(),
                ),
                bid.parse::<u32>().unwrap(),
            )
        })
        .collect();

    hands.sort_by(|(a, _), (b, _)| a.cmp(b));

    let winnings: u32 = hands
        .into_iter()
        .zip(1u32..)
        .map(|((_, bid), rank)| rank * bid)
        .sum();
    println!("{winnings}");
}
