use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading the input file.");
            return;
        }
    };


    let mut hands : Vec<Hand> = input
        .lines()
        .enumerate()
        .map(|(_, line)| get_hand(line))
        .filter_map(|hand| hand)
        .collect();

    hands.sort_by(|a, b| match a.main_order().cmp(&b.main_order()) {
        Ordering::Equal => a.secondary_order().cmp(&b.secondary_order()),
        other => other,
    });

    let result : usize = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| -> usize {
            println!("Hand: {:?} has rank {} with {} bid", hand.cards, idx + 1, hand.bid);
            hand.bid * (idx + 1)
        })
        .sum();

    println!("Result: {}", result);
}

fn get_hand(line: &str) -> Option<Hand> {
    let re = Regex::new(r"([\d\w]+)\s+(\d+)").unwrap();

    let hand = re.captures_iter(line)
        .map(|cap| Hand {
             bid: cap[2].parse::<usize>().unwrap(),
            cards: cap[1].chars().map(|c| map_card_to_value(c)).collect()
        })
        .next();
    
    hand
}

#[derive(Debug)]
struct Hand {
    cards : Vec<usize>,
    bid : usize
}

impl Hand {
    fn secondary_order(&self) -> i64 {
        let mut result : i64 = 0;
        result += self.cards[0] as i64 * 100000000;
        result += self.cards[1] as i64 * 1000000;
        result += self.cards[2] as i64 * 10000;
        result += self.cards[3] as i64 * 100;
        result += self.cards[4] as i64 * 1;

        result
    }

    fn main_order(&self) -> usize {
        let mut counts = HashMap::<usize, usize>::new();
        for card in &self.cards {
            let current = counts.entry(*card).or_insert(0);
            *current += 1;
        }
        
        let mut max_order = 0;

        let mut pair_appears : bool = false;
        let mut three_appears : bool = false;
        
        for (_, value) in counts {
            if value == 5 {
                max_order = max_order.max(6);
            }

            if value == 4 {
                max_order = max_order.max(5);
            }

            if value == 3 {
                max_order = max_order.max(3);
                three_appears = true;
            }

            if value == 2 {
                if pair_appears {
                    max_order = max_order.max(2);
                }

                max_order = max_order.max(1);
                pair_appears = true;
            }
        }

        if three_appears && pair_appears {
            max_order = max_order.max(4);
        }

        max_order
    }
}

fn map_card_to_value(character: char) -> usize {
    let dictionary: std::collections::HashMap<char, usize> = [
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14)
    ]
        .iter()
        .cloned()
        .collect();
        
    return *dictionary.get(&character).unwrap()
}
