use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerRank {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug, Eq)]
struct PokerHand<'a> {
    source: &'a str,
    groups: Vec<(u8, u8)>,
    rank: PokerRank,
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.groups == other.groups
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => Some(other.groups.cmp(&self.groups)),
            c => Some(c),
        }
    }
}

impl<'a> From<&'a str> for PokerHand<'a> {
    fn from(source: &'a str) -> Self {
        let mut counter: HashMap<u8, u8> = HashMap::new();
        let mut suits = HashSet::new();
        source.split_whitespace().for_each(|s| {
            let (face, suit) = s.split_at(s.len() - 1);
            let value: u8 = match face {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                _ => face
                    .parse()
                    .unwrap_or_else(|_| panic!("\"{face}\" is not a valid card face")),
            };
            *counter.entry(value).or_insert(0) += 1;
            suits.insert(suit);
        });
        let mut groups: Vec<(u8, u8)> = counter.drain().collect();
        groups.sort_by_key(|&(value, count)| (Reverse(count), Reverse(value)));

        let counts: Vec<u8> = groups.iter().map(|g| g.1).collect();
        let rank = match counts[..] {
            [1, 1, 1, 1, 1] => {
                let faces: Vec<u8> = groups.iter().map(|g| g.0).collect();
                let mut result = PokerRank::HighCard;
                if faces[..] == [14, 5, 4, 3, 2] {
                    // move the ace
                    groups.remove(0);
                    groups.push((1, 1));
                    result = PokerRank::Straight;
                } else if faces
                    .iter()
                    .copied()
                    .eq((faces[faces.len() - 1]..=faces[0]).rev())
                {
                    result = PokerRank::Straight;
                }

                if suits.len() == 1 {
                    if result == PokerRank::Straight {
                        result = PokerRank::StraightFlush;
                    } else {
                        result = PokerRank::Flush;
                    }
                }
                result
            }
            [4, 1] => PokerRank::FourOfAKind,
            [3, 2] => PokerRank::FullHouse,
            [3, 1, 1] => PokerRank::ThreeOfAKind,
            [2, 2, 1] => PokerRank::TwoPair,
            [2, 1, 1, 1] => PokerRank::Pair,
            _ => PokerRank::HighCard,
        };
        Self {
            source,
            groups,
            rank,
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
#[must_use]
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut ranked: Vec<PokerHand> = hands.iter().map(|h| PokerHand::from(*h)).collect();
    ranked.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater));
    ranked
        .iter()
        .filter(|h| ranked[0].eq(h))
        .map(|h| h.source)
        .collect::<Vec<&'a str>>()
}
