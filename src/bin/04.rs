advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards: Vec<&str> = input.trim_end().split('\n').collect();
    let pow: u32 = cards
        .iter()
        .map(|&card| process_each_card(&card))
        .sum::<u32>();
    Some(pow)
}

pub fn process_each_card(card: &str) -> u32 {
    let card = card.split_once(':').unwrap().1;
    let mut winning_nums = card
        .split_once('|')
        .unwrap()
        .0
        .trim_start()
        .trim_end()
        .split_whitespace()
        .map(|num| u32::from_str_radix(num, 10).unwrap())
        .collect::<Vec<u32>>();
    let mut my_nums = card
        .split_once('|')
        .unwrap()
        .1
        .trim_end()
        .trim_start()
        .split_whitespace()
        .map(|num| u32::from_str_radix(num, 10).unwrap())
        .collect::<Vec<u32>>();
    winning_nums.sort();
    my_nums.sort();
    let mut matches: u32 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    while (i < winning_nums.len()) && (j < my_nums.len()) {
        if (winning_nums[i] == my_nums[j]) {
            matches += 1;
            i += 1;
            j += 1;
        } else if winning_nums[i] < my_nums[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    match matches {
        0 => {
            // println!("No matches");
            0
        }
        _ => {
            // println!("{}, {}", matches, 2_u32.pow(matches - 1));
            2_u32.pow(matches - 1)
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<&str> = input.trim_end().split('\n').collect();
    let mut cards_total: Vec<u32> = vec![1; cards.len()];
    // println!("{:?}", cards.len());
    process_cards(cards, &mut cards_total);
    Some(cards_total.into_iter().sum())
}

pub fn process_cards(cards: Vec<&str>, cards_total: &mut [u32]) {
    for card in cards {
        let line = card.split_once(':').unwrap();
        let card = line.1;
        let card_num = line.0.split_whitespace().last().unwrap();

        // println!("{:?}", card_num);
        let mut winning_nums = card
            .split_once('|')
            .unwrap()
            .0
            .trim_start()
            .trim_end()
            .split_whitespace()
            .map(|num| u32::from_str_radix(num, 10).unwrap())
            .collect::<Vec<u32>>();
        let mut my_nums = card
            .split_once('|')
            .unwrap()
            .1
            .trim_end()
            .trim_start()
            .split_whitespace()
            .map(|num| u32::from_str_radix(num, 10).unwrap())
            .collect::<Vec<u32>>();
        winning_nums.sort();
        my_nums.sort();
        let mut matches: u32 = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;
        while (i < winning_nums.len()) && (j < my_nums.len()) {
            if (winning_nums[i] == my_nums[j]) {
                matches += 1;
                i += 1;
                j += 1;
            } else if winning_nums[i] < my_nums[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        for copy_card in 0..matches {
            cards_total[usize::from_str_radix(card_num, 10).unwrap() + copy_card as usize] +=
                cards_total[usize::from_str_radix(card_num, 10).unwrap() - 1];
            // println!(
            //     "{}",
            //     usize::from_str_radix(card_num, 10).unwrap() + copy_card as usize
            // );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
