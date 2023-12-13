use lazy_static::lazy_static;
use std::{cmp::Ordering, collections::HashMap, u128};
advent_of_code::solution!(7);
lazy_static! {
    static ref card_map: HashMap<char, card_type> = HashMap::from([
        ('A', card_type::A),
        ('K', card_type::K),
        ('Q', card_type::Q),
        ('J', card_type::J),
        ('T', card_type::T),
        ('9', card_type::nine),
        ('8', card_type::eight),
        ('7', card_type::seven),
        ('6', card_type::six),
        ('5', card_type::five),
        ('4', card_type::four),
        ('3', card_type::three),
        ('2', card_type::two),
    ]);
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
enum hand_type {
    none,
    high_card,
    one_pair,
    two_pair,
    three_of_kind,
    full_house,
    four_of_kind,
    five_of_kind,
}
impl hand_type {
    fn new(game: &str) -> Self {
        let mut count = HashMap::new();
        for i in game.chars() {
            count
                .entry(i)
                .and_modify(|e| *e = (*e + 1))
                .or_insert(1_u32);
        }
        let mut hash_vec: Vec<(&char, &u32)> = count.iter().collect();
        // println!("{:?}", hash_vec);
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut hash_vec_iter = hash_vec.iter();
        let (c, count) = hash_vec_iter.next().unwrap();
        match count {
            5 => hand_type::five_of_kind,
            4 => hand_type::four_of_kind,
            3 => {
                let (c, count) = hash_vec_iter.next().unwrap();
                return match count {
                    2 => hand_type::full_house,
                    _ => hand_type::three_of_kind,
                };
            }
            2 => {
                let (c, count) = hash_vec_iter.next().unwrap();
                return match count {
                    2 => hand_type::two_pair,
                    1 => hand_type::one_pair,
                    _ => hand_type::none,
                };
            }
            1 => hand_type::high_card,
            _ => hand_type::none,
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Clone, Copy, Debug)]
enum card_type {
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    T,
    J,
    Q,
    K,
    A,
}

impl card_type {
    fn from(g: &str) -> [Self; 5] {
        let mut cards: [card_type; 5] = [card_type::two; 5];
        for (i, ch) in g.chars().enumerate() {
            // dbg!(ch);
            cards[i] = card_map[&ch].clone();
        }
        return cards;
    }
}

#[derive(Debug)]
struct hand {
    hand_type: hand_type,
    cards: [card_type; 5],
    bid: u32,
}

impl Eq for hand {}
impl Ord for hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => {
                for (i, ch) in self.cards.iter().enumerate() {
                    // dbg!((i, ch, &self.cards), &other.cards);
                    if ch > &other.cards[i] {
                        return Ordering::Greater;
                    } else if ch < &other.cards[i] {
                        return Ordering::Less;
                    } else {
                        continue;
                    }
                }
                return Ordering::Equal;
            }
            Some(s) => {
                return s;
            }
            None => Ordering::Equal,
        }
    }
}
impl PartialOrd for hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type && self.bid == other.bid
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut games: Vec<hand> = input
        .trim_end()
        .split('\n')
        .map(|game| {
            let g: Vec<&str> = game.split_whitespace().collect();
            hand {
                hand_type: hand_type::new(g[0]),
                cards: card_type::from(g[0]),
                bid: str::parse(g[1]).unwrap(),
            }
        })
        .collect();

    games.sort();
    // dbg!(&games);
    let mut s: u128 = 0;
    for (i, game) in games.iter().enumerate() {
        s += ((i + 1) * game.bid as usize) as u128;
        // dbg!(((i + 1) * game.bid as usize));
        // dbg!(((i + 1), game.bid as usize));
    }
    Some(s)
}

lazy_static! {
    static ref card_map2: HashMap<char, card_type2> = HashMap::from([
        ('A', card_type2::A),
        ('K', card_type2::K),
        ('Q', card_type2::Q),
        ('J', card_type2::J),
        ('T', card_type2::T),
        ('9', card_type2::nine),
        ('8', card_type2::eight),
        ('7', card_type2::seven),
        ('6', card_type2::six),
        ('5', card_type2::five),
        ('4', card_type2::four),
        ('3', card_type2::three),
        ('2', card_type2::two),
    ]);
}

pub fn replace_j_with_most_occ(inp: &str) -> String {
    let mut count = HashMap::new();
    for i in inp.chars() {
        count
            .entry(i)
            .and_modify(|e| *e = (*e + 1))
            .or_insert(1_u32);
    }
    let mut hash_vec: Vec<(&char, &u32)> = count.iter().collect();
    // println!("{:?}", hash_vec);
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    if count.contains_key(&'J') {
        let mut most_occ_ch = &'J';
        for i in hash_vec {
            if i.0 != &'J' {
                most_occ_ch = i.0;
                break;
            }
        }
        let inp = inp.replace('J', most_occ_ch.to_string().as_str());
        return inp;
    }
    inp.to_string()
}
#[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
enum hand_type2 {
    none,
    high_card,
    one_pair,
    two_pair,
    three_of_kind,
    full_house,
    four_of_kind,
    five_of_kind,
}
impl hand_type2 {
    fn new(game: &str) -> Self {
        let mut count = HashMap::new();
        let game = replace_j_with_most_occ(game);
        for i in game.chars() {
            count
                .entry(i)
                .and_modify(|e| *e = (*e + 1))
                .or_insert(1_u32);
        }
        let mut hash_vec: Vec<(&char, &u32)> = count.iter().collect();
        // println!("{:?}", hash_vec);
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut hash_vec_iter = hash_vec.iter();
        let (c, count) = hash_vec_iter.next().unwrap();
        match count {
            5 => hand_type2::five_of_kind,
            4 => hand_type2::four_of_kind,
            3 => {
                let (c, count) = hash_vec_iter.next().unwrap();
                return match count {
                    2 => hand_type2::full_house,
                    _ => hand_type2::three_of_kind,
                };
            }
            2 => {
                let (c, count) = hash_vec_iter.next().unwrap();
                return match count {
                    2 => hand_type2::two_pair,
                    1 => hand_type2::one_pair,
                    _ => hand_type2::none,
                };
            }
            1 => hand_type2::high_card,
            _ => hand_type2::none,
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Clone, Copy, Debug)]
enum card_type2 {
    J,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    T,
    Q,
    K,
    A,
}

impl card_type2 {
    fn from(g: &str) -> [Self; 5] {
        let mut cards: [card_type2; 5] = [card_type2::J; 5];
        for (i, ch) in g.chars().enumerate() {
            // dbg!(ch);
            cards[i] = card_map2[&ch].clone();
        }
        return cards;
    }
}

#[derive(Debug)]
struct hand2 {
    hand_type: hand_type2,
    cards: [card_type2; 5],
    bid: u32,
}

impl Eq for hand2 {}
impl Ord for hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => {
                for (i, ch) in self.cards.iter().enumerate() {
                    // dbg!((i, ch, &self.cards), &other.cards);
                    if ch > &other.cards[i] {
                        return Ordering::Greater;
                    } else if ch < &other.cards[i] {
                        return Ordering::Less;
                    } else {
                        continue;
                    }
                }
                return Ordering::Equal;
            }
            Some(s) => {
                return s;
            }
            None => Ordering::Equal,
        }
    }
}
impl PartialOrd for hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type && self.bid == other.bid
    }
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut games: Vec<hand2> = input
        .trim_end()
        .split('\n')
        .map(|game| {
            let g: Vec<&str> = game.split_whitespace().collect();
            hand2 {
                hand_type: hand_type2::new(g[0]),
                cards: card_type2::from(g[0]),
                bid: str::parse(g[1]).unwrap(),
            }
        })
        .collect();

    games.sort();
    // dbg!(&games);
    let mut s: u128 = 0;
    for (i, game) in games.iter().enumerate() {
        s += ((i + 1) * game.bid as usize) as u128;
        // dbg!(((i + 1) * game.bid as usize));
        // dbg!(((i + 1), game.bid as usize));
    }
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
