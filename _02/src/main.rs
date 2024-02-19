use std::fs;

fn main() {
    let games = parse_input();
    println!("Part 1: {}", sum_satisfied_game_id(games.clone()));
    println!("Part 2: {}", sum_bag_power(games.clone()));
}

fn parse_input() -> Vec<(i32, Vec<[i32; 3]>)> {
    let result = fs::read_to_string("data/input.txt");
    if result.is_err() {
        panic!("Error reading file: {:?}", result.err());
    }

    let contents = result.unwrap();
    return contents
        .split("\n")
        .map(|s| parse_game(s.to_string()))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect();
}

fn parse_bag(bag_repr: String) -> [i32; 3] {
    let mut num_cubes = [0, 0, 0];
    let tokens = bag_repr.split(", ");
    for token in tokens {
        let (str_count, color) = token.split_once(" ").unwrap();
        let count = str_count.parse::<i32>().unwrap();
        match color {
            "red" => num_cubes[0] += count,
            "green" => num_cubes[1] += count,
            "blue" => num_cubes[2] += count,
            _ => panic!("Invalid color: {}", color),
        }
    }

    return num_cubes;
}

fn parse_game(input: String) -> Option<(i32, Vec<[i32; 3]>)> {
    let tokens = input.split_once(": ");
    if tokens.is_none() {
        return None;
    }

    let (str_game_id, str_bags) = tokens.unwrap();
    let game_id = str_game_id[5..].parse::<i32>().unwrap();
    let bags = str_bags
        .split("; ")
        .map(|s| parse_bag(s.to_string()))
        .collect();

    return Some((game_id, bags));
}

fn sum_satisfied_game_id(games: Vec<(i32, Vec<[i32; 3]>)>) -> i32 {
    let mut game_count = 0;
    for (game_id, bags) in games {
        if is_satisfied_bag(&bags) {
            game_count += game_id;
        }
    }

    return game_count;
}

fn sum_bag_power(games: Vec<(i32, Vec<[i32; 3]>)>) -> i32 {
    let mut power = 0;
    for (_, bags) in games {
        power += calculate_power(&bags)
    }

    return power;
}

fn is_satisfied_bag(bags: &Vec<[i32; 3]>) -> bool {
    let num_cubes = find_min_num_cubes(bags);
    return num_cubes[0] <= 12 && num_cubes[1] <= 13 && num_cubes[2] <= 14;
}

fn calculate_power(bags: &Vec<[i32; 3]>) -> i32 {
    let num_cubes = find_min_num_cubes(bags);
    return num_cubes[0] * num_cubes[1] * num_cubes[2];
}

fn find_min_num_cubes(bags: &Vec<[i32; 3]>) -> [i32; 3] {
    let mut num_cubes = [0, 0, 0];
    for bag in bags {
        for i in 0..bag.len() {
            num_cubes[i] = num_cubes[i].max(bag[i]);
        }
    }

    return num_cubes;
}
