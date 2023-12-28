// Note: the order of the variants matters to derive it for an enum: lowest to the highest!

use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

/// Camel Cards
pub fn solver(part: Part, input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').context("No space delimiter")?;
            let bid: u32 = bid.parse()?;
            let cards = cards.chars().map(Card::try_from).ok_collect_array()?;
            Ok((cards, bid))
        })
        .process_results(|it| match part {
            Part1 => it.total_bid(),
            Part2 => it
                .map(|(cards, bid)| (cards.map(Card::weak_joker), bid))
                .total_bid(),
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Joker,
    Q,
    K,
    A,
}

/// Joker being none is the lowest.
///
/// Note that `Some(Card::Joker)` is representable but is a wrong state.
type WeakJokerCard = Option<Card>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

trait GetHand {
    fn hand(self) -> Hand;
}

trait TotalBid {
    fn total_bid(self) -> u32;
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(ch: char) -> Result<Self> {
        Ok(match ch {
            '2' => Self::N2,
            '3' => Self::N3,
            '4' => Self::N4,
            '5' => Self::N5,
            '6' => Self::N6,
            '7' => Self::N7,
            '8' => Self::N8,
            '9' => Self::N9,
            'T' => Self::T,
            'J' => Self::Joker,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => bail!("Not 2-9TJQKA but {}", ch),
        })
    }
}

impl Card {
    #[inline]
    fn weak_joker(self) -> WeakJokerCard {
        (self != Self::Joker).then_some(self)
    }
}

impl GetHand for [Card; 5] {
    fn hand(mut self) -> Hand {
        self.sort();
        // Count the grouped cards.
        let mut counts = self
            .iter()
            .dedup_with_count()
            .map(|(count, _)| count)
            .collect_vec();
        // Identify the hand from the counts.
        counts.sort_unstable();
        counts.reverse();
        match counts[..] {
            [5] => Hand::FiveOfAKind,
            [4, 1] => Hand::FourOfAKind,
            [3, 2] => Hand::FullHouse,
            [3, 1, 1] => Hand::ThreeOfAKind,
            [2, 2, 1] => Hand::TwoPairs,
            [2, 1, 1, 1] => Hand::OnePair,
            [1, 1, 1, 1, 1] => Hand::HighCard,
            _ => unreachable!(),
        }
    }
}

impl GetHand for [WeakJokerCard; 5] {
    fn hand(mut self) -> Hand {
        // Similar but without the joker.
        self.sort();
        let mut counts = self
            .iter()
            .filter(|c| c.is_some())
            .dedup_with_count()
            .map(|(count, _)| count)
            .collect_vec();
        counts.sort_unstable();
        counts.reverse();
        // The biggest count does not matter as it is increased by the number of jokers.
        // And there is one edge case: 5 jokers!
        match counts[..] {
            [] | [_] => Hand::FiveOfAKind,
            [_, 1] => Hand::FourOfAKind,
            [_, 2] => Hand::FullHouse,
            [_, 1, 1] => Hand::ThreeOfAKind,
            [_, 2, 1] => Hand::TwoPairs,
            [_, 1, 1, 1] => Hand::OnePair,
            [1, 1, 1, 1, 1] => Hand::HighCard,
            _ => unreachable!(),
        }
    }
}

impl<I, C> TotalBid for I
where
    I: Iterator<Item = (C, u32)>,
    C: GetHand + Copy + Ord,
{
    #[allow(clippy::cast_possible_truncation)] // There is not `u32::MAX` lines.
    #[inline]
    fn total_bid(self) -> u32 {
        self.map(|(cards, bid)| (cards.hand(), cards, bid))
            .sorted()
            .enumerate()
            .map(|(idx, (_, _, bid))| (idx as u32 + 1) * bid)
            .sum()
    }
}

test_solver! {
    "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
" => (6440, 5905),
    include_input!(23 07) => (248453531, 248781813),
}
