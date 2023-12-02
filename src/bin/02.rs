advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let rc = 12_u32;
    let gc = 13_u32;
    let bc = 14_u32;
    let games_cube_count: u32 = input
        .trim_end()
        .split('\n')
        .map(|line| vaild_game_ids(line, rc, gc, bc).unwrap_or(0))
        .sum();
    // for i in games_cube_count {
    //     println!("{:?}", vaild_game_ids(i, rc, gc, bc));
    // }
    Some(games_cube_count)
    // None
}
struct game {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

pub fn vaild_game_ids(line: &str, rc: u32, gc: u32, bc: u32) -> Option<u32> {
    let game = line.split(':').collect::<Vec<&str>>();
    let game_id = game[0].split(' ').nth(1).unwrap();

    // println!("{:?}", game_id);
    let game_vals = game[1].trim_start().split(';').collect::<Vec<&str>>();
    let mut sum_check = 0;
    for shown_cubes in game_vals {
        let turns: Vec<&str> = shown_cubes.split(',').collect();
        let cubes_color: u32 = turns
            .iter()
            .map(|colr: &&str| {
                let x = &colr.trim_start().split(' ').collect::<Vec<&str>>();
                // println!("{:?}", x[1]);
                let v: u32 = match x[1].trim_start() {
                    "blue" => {
                        if u32::from_str_radix(x[0], 10).unwrap() > bc {
                            1
                        } else {
                            0
                        }
                    }
                    "red" => {
                        if u32::from_str_radix(x[0], 10).unwrap() > rc {
                            1
                        } else {
                            0
                        }
                    }
                    "green" => {
                        if u32::from_str_radix(x[0], 10).unwrap() > gc {
                            1
                        } else {
                            0
                        }
                    }
                    _ => 1,
                };
                v
            })
            .sum();
        sum_check += cubes_color;
    }
    if sum_check == 0 {
        // println!("{:?}", game_id);
        return Some(u32::from_str_radix(game_id, 10).unwrap());
    } else {
        return None;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let s = input
        .trim_end()
        .split('\n')
        .map(|line| power_of_cube(&line).unwrap_or(0))
        .sum();
    Some(s)
}

pub fn power_of_cube(line: &str) -> Option<u32> {
    let game = line.split(':').collect::<Vec<&str>>();
    let mut bc = 0;
    let mut rc = 0;
    let mut gc = 0;
    // println!("{:?}", game_id);
    let game_vals = game[1].trim_start().split(';').collect::<Vec<&str>>();
    for shown_cubes in game_vals {
        let turns: Vec<&str> = shown_cubes.split(',').collect();

        // let cubes_color: (u32, u32, u32) =
        for colr in turns.iter() {
            let x = &colr.trim_start().split(' ').collect::<Vec<&str>>();
            // println!("{:?}", x);
            let cube_count = u32::from_str_radix(x[0], 10).unwrap();
            match x[1].trim_start() {
                "blue" => {
                    if cube_count > bc {
                        bc = cube_count;
                    }
                }
                "red" => {
                    if cube_count > rc {
                        rc = cube_count;
                    }
                }
                "green" => {
                    if cube_count > gc {
                        gc = cube_count;
                    }
                }
                _ => (),
            };
        }
    }
    return Some(rc * gc * bc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
