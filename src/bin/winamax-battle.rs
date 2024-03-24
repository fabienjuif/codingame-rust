// https://www.codingame.com/ide/puzzle/winamax-battle

use std::{collections::VecDeque, fmt, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(PartialEq, PartialOrd, Clone)]
enum Card {
    Two = 2,
    Three,
    Foor,
    Five,
    Six,
    Seven,
    Height,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Foor => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Height => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::J => "J",
            Self::Q => "Q",
            Self::K => "K",
            Self::A => "A",
        })
    }
}

impl Card {
    pub fn from_str(s: &str) -> Self {
        let mut card_str = s;
        if s.len() > 1 {
            card_str = s.get(0..(s.len() - 1)).unwrap();
        }
        match card_str {
            "2" => return Card::Two,
            "3" => return Card::Three,
            "4" => return Card::Foor,
            "5" => return Card::Five,
            "6" => return Card::Six,
            "7" => return Card::Seven,
            "8" => return Card::Height,
            "9" => return Card::Nine,
            "10" => return Card::Ten,
            "J" => return Card::J,
            "Q" => return Card::Q,
            "K" => return Card::K,
            "A" => return Card::A,
            _ => {
                panic!("unknown card: {} ({})", card_str, s)
            }
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of cards for player 1
    let mut p1_cards = VecDeque::<Card>::new();
    let mut p2_cards = VecDeque::<Card>::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let cardp_1 = input_line.trim().to_string(); // the n cards of player 1
        p1_cards.push_back(Card::from_str(&cardp_1));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = parse_input!(input_line, i32); // the number of cards for player 2
    for _ in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let cardp_2 = input_line.trim().to_string(); // the m cards of player 2
        p2_cards.push_back(Card::from_str(&cardp_2));
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    eprintln!("p1_cards: {:?}", p1_cards);
    eprintln!("p2_cards: {:?}", p2_cards);

    //
    let mut round = 0;
    let mut p1_stack = VecDeque::<Card>::new();
    let mut p2_stack = VecDeque::<Card>::new();
    loop {
        // eprintln!("{} -> {:?} | {:?}", round, p1_cards, p2_cards);

        let Some(p1_card) = p1_cards.pop_front() else {
            if p1_stack.len() + p2_stack.len() > 0 {
                println!("PAT")
            } else {
                println!("2 {}", round);
            }
            return;
        };
        let Some(p2_card) = p2_cards.pop_front() else {
            if p1_stack.len() + p2_stack.len() > 0 {
                println!("PAT")
            } else {
                println!("1 {}", round);
            }
            return;
        };

        // eprintln!("p1_card: {:?}", p1_card);
        // eprintln!("p2_card: {:?}", p2_card);

        p1_stack.push_back(p1_card.clone());
        p2_stack.push_back(p2_card.clone());

        if p1_card == p2_card {
            for _ in 0..3 {
                let Some(card) = p1_cards.pop_front() else {
                    println!("PAT");
                    return;
                };
                p1_stack.push_back(card);
            }
            for _ in 0..3 {
                let Some(card) = p2_cards.pop_front() else {
                    println!("PAT");
                    return;
                };
                p2_stack.push_back(card);
            }
            eprintln!("stack: p1={:?} | p2={:?}", p1_stack, p2_stack);
        } else {
            round += 1;
            if p2_card > p1_card {
                while p1_stack.len() > 0 {
                    p2_cards.push_back(p1_stack.pop_front().unwrap());
                }
                while p2_stack.len() > 0 {
                    p2_cards.push_back(p2_stack.pop_front().unwrap());
                }
            } else if p1_card > p2_card {
                while p1_stack.len() > 0 {
                    p1_cards.push_back(p1_stack.pop_front().unwrap());
                }
                while p2_stack.len() > 0 {
                    p1_cards.push_back(p2_stack.pop_front().unwrap());
                }
            }
        }
    }
}
