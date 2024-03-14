fn read_input() -> String {
    std::fs::read_to_string("data/input.txt")
        .unwrap()
        .trim()
        .to_string()
}

fn parse_values(s: &str) -> Vec<i64> {
    let (_, repr) = s.split_once(":").unwrap();
    repr.split(' ')
        .filter(|repr| !repr.is_empty())
        .map(|repr| repr.parse::<i64>().unwrap())
        .collect()
}

fn parse(content: &String) -> (Vec<i64>, Vec<i64>) {
    let mut lines = content.lines();
    let times = parse_values(lines.next().unwrap());
    let distances = parse_values(lines.next().unwrap());
    (times, distances)
}
pub fn accumulated_input(values: &Vec<i64>) -> i64 {
    values
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap()
}

pub fn input() -> (Vec<i64>, Vec<i64>) {
    parse(&read_input())
}
