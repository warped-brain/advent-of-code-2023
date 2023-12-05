advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_lines = input
        .trim_end()
        .trim_start()
        .split('\n')
        .collect::<Vec<&str>>();
    let seeds = parsed_lines[0]
        .split_whitespace()
        .skip(1)
        .map(|sn| u64::from_str_radix(sn, 10).unwrap())
        .collect::<Vec<u64>>();
    println!("seeds: {:?}", seeds);
    let mut i: usize = 1;
    let mut all_mappings: Vec<map_table> = Vec::new();
    while i < parsed_lines.len() {
        if parsed_lines[i] == "" {
            i += 2;
            continue;
        }
        let mut table: map_table = map_table::new();
        while (i < parsed_lines.len()) && (parsed_lines[i] != "") {
            let vals = parsed_lines[i]
                .split_whitespace()
                .map(|val| u64::from_str_radix(val, 10).unwrap())
                .collect::<Vec<u64>>();
            table.append_mapping(range_vec::new(vals[0], vals[1], vals[2]));

            i += 1;
        }

        all_mappings.push(table);
    }
    println!("{:?}", seeds);
    let mut min_loc = loc_find(seeds[0], &all_mappings);
    for seed in 1..seeds.len() {
        min_loc = u64::min(loc_find(seeds[seed], &all_mappings), min_loc);
    }
    Some(min_loc)
}

fn loc_find(seed: u64, mappings: &Vec<map_table>) -> u64 {
    let mut out = mappings[0].find_mapping(seed);
    for i in 1..mappings.len() {
        out = mappings[i].find_mapping(out);
    }
    out
}

#[derive(Debug)]
struct range_vec {
    src_start: u64,
    dst_start: u64,
    range: u64,
}
impl range_vec {
    fn in_range(&self, query: u64) -> Option<u64> {
        if query >= self.src_start && query < self.src_start + self.range {
            Some(query - self.src_start + self.dst_start)
        } else {
            None
        }
    }
    fn new(dst: u64, src: u64, range: u64) -> Self {
        range_vec {
            src_start: src,
            dst_start: dst,
            range: range,
        }
    }
}

#[derive(Debug)]
struct map_table {
    mappings: Vec<range_vec>,
}
impl map_table {
    fn find_mapping(&self, query: u64) -> u64 {
        let mut mapped_val = query;
        for range in &self.mappings {
            match range.in_range(query) {
                Some(val) => {
                    mapped_val = val;
                }
                None => continue,
            }
        }
        mapped_val
    }
    fn append_mapping(&mut self, mapping: range_vec) {
        &self.mappings.push(mapping);
    }
    fn new() -> Self {
        map_table {
            mappings: Vec::new(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_lines = input
        .trim_end()
        .trim_start()
        .split('\n')
        .collect::<Vec<&str>>();
    let seeds = parsed_lines[0]
        .split_whitespace()
        .skip(1)
        .map(|sn| u64::from_str_radix(sn, 10).unwrap())
        .collect::<Vec<u64>>();
    println!("seeds: {:?}", seeds);
    let mut i: usize = 1;
    let mut all_mappings: Vec<map_table> = Vec::new();
    while i < parsed_lines.len() {
        if parsed_lines[i] == "" {
            i += 2;
            continue;
        }
        let mut table: map_table = map_table::new();
        while (i < parsed_lines.len()) && (parsed_lines[i] != "") {
            let vals = parsed_lines[i]
                .split_whitespace()
                .map(|val| u64::from_str_radix(val, 10).unwrap())
                .collect::<Vec<u64>>();
            table.append_mapping(range_vec::new(vals[0], vals[1], vals[2]));

            i += 1;
        }

        all_mappings.push(table);
    }
    let mut min_loc = u64::MAX;
    let compressed_seeds = compress_seeds(&seeds);
    let mut x = 1;
    let n = compressed_seeds.len();
    for range in compressed_seeds {
        println!("range: {x} out of {n}");
        for j in range.0..range.1 {
            min_loc = u64::min(loc_find(j, &all_mappings), min_loc);
        }
        x += 1;
    }
    Some(min_loc)
}
pub fn compress_seeds(seeds: &Vec<u64>) -> Vec<(u64, u64)> {
    let mut range_seeds: Vec<(u64, u64)> = Vec::with_capacity(seeds.len() / 2);
    for j in (0..seeds.len()).step_by(2) {
        range_seeds.push((seeds[j], (seeds[j] + seeds[j + 1])));
    }

    let mut compressed_range: Vec<(u64, u64)> = Vec::new();
    range_seeds.sort_by(|a, b| a.0.cmp(&b.0));

    let mut i: usize = 1;
    let mut current_range = range_seeds[0];
    while i < range_seeds.len() {
        if range_seeds[i].0 <= range_seeds[i - 1].1 {
            current_range.1 = current_range.1.max(range_seeds[i].1);
        } else {
            compressed_range.push(current_range);
            current_range = range_seeds[i];
        }
        i += 1;
    }
    compressed_range.push(current_range);
    compressed_range
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
