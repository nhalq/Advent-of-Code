mod aoc_io;

fn race_test(race_time: i64, race_distance: i64) -> i64 {
    let mut beateds = 0;
    for hold_time in 1..race_time {
        let velocity = hold_time;
        let remaining_time = race_time - hold_time;

        let actual_distance = velocity * remaining_time;
        if actual_distance > race_distance {
            beateds += 1;
        }
    }

    beateds
}

fn solve(race_times: Vec<i64>, race_distances: Vec<i64>) -> i64 {
    let mut n_wins = 1;
    for (race_time, race_distance) in race_times.iter().zip(race_distances.iter()) {
        n_wins *= race_test(*race_time, *race_distance);
    }

    n_wins
}

fn main() {
    let (race_times, race_distances) = aoc_io::input();
    let arace_time = aoc_io::accumulated_input(&race_times);
    let arace_distance = aoc_io::accumulated_input(&race_distances);

    println!("Part 1: {}", solve(race_times, race_distances));
    println!("Part 2: {}", solve(vec![arace_time], vec![arace_distance]));
}
