use itertools::{EitherOrBoth, Itertools};
use std::{cmp::Ordering, collections::HashMap};

type Hand<'a> = (usize, &'a str, usize);

pub fn part_one(input: &str) -> String {
    let mut hands = input
        .lines()
        .map(parse_hand)
        .map(|(hand, bid)| {
            let score = find_type(&card_counts(hand.chars()), 0);
            (score, hand, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|l, r| {
        compare_hands(
            l,
            r,
            &[
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ],
        )
    });
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid)
        .sum::<usize>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    let mut hands = input
        .lines()
        .map(parse_hand)
        .map(|(hand, bid)| {
            let jokers = hand.chars().filter(|c| c == &'J').count();
            let score = find_type(&card_counts(hand.chars().filter(|c| c != &'J')), jokers);
            (score, hand, bid)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|l, r| {
        compare_hands(
            l,
            r,
            &[
                'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
            ],
        )
    });
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid)
        .sum::<usize>()
        .to_string()
}

const HAND_TYPES: &[&[usize; 2]; 8] = &[
    &[5, 0],
    &[4, 0],
    &[3, 2],
    &[3, 0],
    &[2, 2],
    &[2, 0],
    &[1, 0],
    &[0, 0],
];

fn compare_hands(
    (lscore, lhand, _): &Hand,
    (rscore, rhand, _): &Hand,
    card_ord: &[char],
) -> Ordering {
    match lscore.cmp(rscore) {
        Ordering::Equal => {
            for (lc, rc) in lhand.chars().zip(rhand.chars()) {
                if lc != rc {
                    let lval = card_ord.iter().position(|c| c == &lc).unwrap();
                    let rval = card_ord.iter().position(|c| c == &rc).unwrap();
                    return lval.cmp(&rval);
                }
            }
            Ordering::Equal
        }
        ord => ord,
    }
}

fn find_type(card_counts: &[usize], jokers: usize) -> usize {
    HAND_TYPES.len()
        - HAND_TYPES
            .iter()
            .enumerate()
            .find(|(_, &h_type)| {
                let mut available_jokers = jokers;
                h_type
                    .iter()
                    .zip_longest(card_counts.iter())
                    .all(|pair| match pair {
                        EitherOrBoth::Both(s, h) => {
                            if s <= &(h + available_jokers) {
                                let used_jokers = if h < s { s - h } else { 0 };
                                available_jokers -= used_jokers;
                                true
                            } else {
                                false
                            }
                        }
                        EitherOrBoth::Left(s) => {
                            if s <= &available_jokers {
                                available_jokers -= *s;
                                true
                            } else {
                                false
                            }
                        }
                        _ => true,
                    })
            })
            .unwrap()
            .0
}

fn card_counts<I: Iterator<Item = char>>(hand: I) -> Vec<usize> {
    let mut counts = hand
        .fold(HashMap::new(), |mut hand, card| {
            match hand.get_mut(&card) {
                Some(count) => *count += 1,
                None => {
                    hand.insert(card, 1);
                }
            }
            hand
        })
        .into_values()
        .collect::<Vec<_>>();

    counts.sort_by(|l, r| r.cmp(l));
    counts
}

fn parse_hand(line: &str) -> (&str, usize) {
    match line.split(' ').collect::<Vec<_>>()[..] {
        [hand, bid] => (hand, bid.parse().unwrap()),
        _ => unreachable!(),
    }
}
