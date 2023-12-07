use std::{collections::HashMap, fmt::Display};

pub fn part_1(input: &str) -> usize {
    let mut hands = parse_input(input);

    hands.sort();

    let mut total: usize = 0;

    for (i, hand) in hands.iter().rev().enumerate() {
        total += (i + 1) * hand.bet;
    }

    println!("{total}");
    total
}

#[derive(Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
    bet: usize,
    value: Value,
}

impl Hand {
    fn new(cards: Vec<Card>, bet: usize) -> Self {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        cards
            .iter()
            .for_each(|card| *counts.entry(*card).or_default() += 1);

        let mut value: Option<Value> = None;

        for (_card, count) in counts {
            match (count, &value) {
                (5, None) => value = Some(Value::FiveKind),
                (4, None | Some(Value::Highest)) => value = Some(Value::FourKind),
                (3, None | Some(Value::Highest)) => value = Some(Value::ThreeKind),
                (3, Some(Value::OnePair)) => value = Some(Value::FullHouse),
                (2, None | Some(Value::Highest)) => value = Some(Value::OnePair),
                (2, Some(Value::OnePair)) => value = Some(Value::TwoPair),
                (2, Some(Value::ThreeKind)) => value = Some(Value::FullHouse),
                (1, None) => value = Some(Value::Highest),
                (1, Some(Value::FiveKind)) => unreachable!(),
                (1, _) => (),
                (_, _) => unreachable!(),
            }
        }

        Self {
            cards,
            bet,
            value: value.unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(std::cmp::Ordering::Equal) => (),
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}, {:?}, {:?} => {:?} ({})",
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4],
            self.value,
            self.bet
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Value {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    Highest,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    As,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        let mut cards: Vec<Card> = Vec::new();
        let (card_list, bet) = line.trim().split_once(' ').unwrap();
        for (i, c) in card_list.char_indices() {
            if i > 4 {
                panic!()
            }
            match c {
                'A' => cards.push(Card::As),
                'K' => cards.push(Card::King),
                'Q' => cards.push(Card::Queen),
                'J' => cards.push(Card::Jack),
                'T' => cards.push(Card::Ten),
                '9' => cards.push(Card::Nine),
                '8' => cards.push(Card::Eight),
                '7' => cards.push(Card::Seven),
                '6' => cards.push(Card::Six),
                '5' => cards.push(Card::Five),
                '4' => cards.push(Card::Four),
                '3' => cards.push(Card::Three),
                '2' => cards.push(Card::Two),
                _ => unreachable!(),
            }
        }
        hands.push(Hand::new(cards, bet.trim().parse::<usize>().unwrap()))
    }

    hands
}
