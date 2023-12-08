use advent::util::load_file;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Hand {
    value: String, 
    bet: i32
}

// The order of card ranking for stacking hands of similar size
const CARD_RANK: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

const FIVE_OF_A_KIND: &str = "five";
const FOUR_OF_A_KIND: &str = "four";
const THREE_OF_A_KIND: &str = "three";
const ONE_PAIR: &str = "pair";
const HIGH_CARD: &str = "high";
const TWO_PAIR: &str = "two_pair";
const FULL_HOUSE: &str = "full";


fn main() {

    // The order in which hands should be sorted
    let rankings: Vec<&str> = Vec::from([
        HIGH_CARD,
        ONE_PAIR, 
        TWO_PAIR, 
        THREE_OF_A_KIND, 
        FULL_HOUSE, 
        FOUR_OF_A_KIND,        
        FIVE_OF_A_KIND
    ]);

    let mut hands_by_type: HashMap<&str, Vec<Hand>> = HashMap::new();
    
    // Create an empty vector for each type of hand
    for key in &rankings {
        hands_by_type.insert(*key, Vec::<Hand>::new());
    }

    // Store each line in the appropriate vector for type of hand it is
    for line in load_file() {
        let mut parts = line.split_whitespace();
        let hand = Hand {
            value: parts.next().unwrap_or_default().to_string(),
            bet: parts.next().unwrap_or_default().parse::<i32>().unwrap()
        };
        let hand_type: String = get_hand_type(&hand.value);  
        hands_by_type.get_mut(&hand_type[..]).expect("Bleh").push(hand);
    }

    // Create a hash map for quick lookups of character to rank
    let rank_map: HashMap<char, usize> = CARD_RANK.iter().enumerate().map(|(i, &c)| (c, i)).collect();

    let mut total: i32 = 0;
    let mut bet_rank: i32 = 0;

    // In order of rankings, go through each hand type
    for kind in rankings {

        let mut similar_hands: Vec<Hand> = hands_by_type.get(&kind).unwrap().to_vec();

        // for all the hands of a single type, sort them based on the rank map/card rank
        similar_hands.sort_by(|a, b| {
            for (char_a, char_b) in a.value.chars().zip(b.value.chars()) {
                let rank_a = rank_map.get(&char_a).unwrap_or(&usize::MAX);
                let rank_b = rank_map.get(&char_b).unwrap_or(&usize::MAX);
        
                if rank_a != rank_b {
                    return rank_b.cmp(rank_a);
                }
            }
            a.value.len().cmp(&b.value.len())
        });

        // Once they are sorted, add them to the total, multiplied by their rank
        for hand in similar_hands {
            bet_rank += 1;
            total += hand.bet * bet_rank;
        }
    }

    println!("Total {}", total);
}

fn get_hand_type(hand: &str) -> String {

    let mut numbers = count_characters(hand);

    let jokers = numbers.remove(&'J').unwrap_or_default();

    let val: &str = match numbers.values().len() {
        // If there's only a single character (or only jokers)
        0 | 1 => FIVE_OF_A_KIND,  

        // If their are two characters, it's 4/1 or 3/2
        2 => { 
            if numbers.values().any(|&val| val + jokers == 4) { FOUR_OF_A_KIND }
            else { FULL_HOUSE}
        },
        
        // If there's only a single character
        3 => {
            if numbers.values().any(|&val| val + jokers == 3){ THREE_OF_A_KIND }
            // Interesting observation: If you have a joker, you never have two pair
            else { TWO_PAIR }
        }
        4 => ONE_PAIR,
        _ => HIGH_CARD,
    };

    val.to_string()

}

fn count_characters(s: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

  