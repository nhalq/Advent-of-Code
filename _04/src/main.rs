use std::collections::HashSet;

fn input() -> Vec<(HashSet<i32>, Vec<i32>)> {
    let content = std::fs::read_to_string("data/input.txt")
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .to_string();

    content
        .split('\n')
        .map(str_to_card)
        .collect::<Vec<(HashSet<i32>, Vec<i32>)>>()
}

fn str_to_card(card: &str) -> (HashSet<i32>, Vec<i32>) {
    let (_, numbers) = card.split_once(':').unwrap();
    let (wins, reveals) = numbers.split_once('|').unwrap();

    (
        HashSet::from_iter(str_to_sequence(wins)),
        str_to_sequence(reveals),
    )
}

fn str_to_sequence(sequence: &str) -> Vec<i32> {
    sequence
        .split(' ')
        .map(|number| number.parse::<i32>())
        .filter(|result| result.is_ok())
        .map(|number| number.unwrap())
        .collect::<Vec<i32>>()
}

fn count_matches(wins: &HashSet<i32>, reveals: &Vec<i32>) -> i32 {
    reveals
        .iter()
        .map(|n| if wins.contains(n) { 1 } else { 0 })
        .sum::<i32>()
}

fn get_matches(cards: &Vec<(HashSet<i32>, Vec<i32>)>) -> Vec<i32> {
    cards
        .iter()
        .map(|(wins, reveals)| count_matches(wins, reveals))
        .collect::<Vec<i32>>()
}

fn get_card_scores(matches: &Vec<i32>) -> Vec<i32> {
    matches
        .iter()
        .map(|n_matches| {
            if (*n_matches) == 0 {
                return 0;
            }

            1 << (n_matches - 1)
        })
        .collect::<Vec<i32>>()
}

fn get_card_copies(matches: &Vec<i32>) -> Vec<i32> {
    let mut copies = vec![1; matches.len()];
    for (u, n_matches) in matches.iter().enumerate() {
        let max_v = matches.len().min(u + n_matches.clone() as usize + 1);
        for v in (u + 1)..max_v {
            copies[v] += copies[u];
        }
    }

    copies
}

fn main() {
    let cards = input();
    let matches = get_matches(&cards);

    let scores = get_card_scores(&matches);
    let copies = get_card_copies(&matches);

    println!("Part 1: {}", scores.iter().sum::<i32>());
    println!("Part 2: {}", copies.iter().sum::<i32>());
}
