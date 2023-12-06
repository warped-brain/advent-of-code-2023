advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (time, distance) = input.split_once('\n').unwrap();
    let time_vec = time
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| u64::from_str_radix(n, 10).unwrap())
        .collect::<Vec<u64>>();
    // let distance = input.split(": ").nth(1);
    let dist_vec = distance
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| u64::from_str_radix(n, 10).unwrap())
        .collect::<Vec<u64>>();
    // println!("{:?}", time_vec);
    let mut multval = 1_u64;
    for i in 0..time_vec.len() {
        multval = multval * calc_n_ways_bs(time_vec[i], dist_vec[i]);
    }
    Some(multval)
}

pub fn calc_n_ways(time: u64, dist: u64) -> u64 {
    let mut count = 0;
    for hold_time in 0..time {
        if ((time - hold_time) * hold_time) > dist {
            count += 1;
        }
    }
    count
}

pub fn calc_n_ways_bs(time: u64, dist: u64) -> u64 {
    let mut low = 0_u64;
    let mut high = time;
    let mut ans: u64;
    while low <= high {
        let mid = low + high >> 1;
        if ((time - mid) * mid) > dist {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    time + 1 - 2 * low
}

pub fn part_two(input: &str) -> Option<u64> {
    let (time, distance) = input.split_once('\n').unwrap();
    let time_vec = time
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let time_val = str::parse::<u64>(time_vec.concat().trim()).unwrap();
    // let distance = input.split(": ").nth(1);
    let dist_vec = distance
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let dist_val = str::parse::<u64>(dist_vec.concat().trim()).unwrap();
    // println!("{:?}", time_vec);
    let mut multval = calc_n_ways_bs(time_val, dist_val);
    // for i in 0..time_val {
    //     multval = multval + calc_n_ways(time_vec[i], dist_vec[i]);
    // }
    Some(multval)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
