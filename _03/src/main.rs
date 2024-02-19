use std::collections::HashSet;

struct EngineSchematic {
    num_rows: i32,
    num_columns: i32,
    schematic: Vec<Vec<char>>,
}

type Location = (i32, i32);
type NumberLocation = (i32, Location);

fn input() -> EngineSchematic {
    let content = std::fs::read_to_string("data/input.txt")
        .unwrap()
        .strip_suffix('\n')
        .unwrap()
        .to_string();

    let schematic = content
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    return EngineSchematic {
        num_rows: schematic.len() as i32,
        num_columns: schematic[0].len() as i32,
        schematic: schematic,
    };
}

fn main() {
    let engine = input();
    println!("Part 1: {}", engine.solve_part1());
    println!("Part 2: {}", engine.solve_part2());
}

impl EngineSchematic {
    fn in_range(&self, row_idx: i32, column_idx: i32) -> bool {
        return (0 <= row_idx && row_idx < self.num_rows)
            && (0 <= column_idx && column_idx < self.num_columns);
    }

    fn is_digit(&self, row_idx: i32, column_idx: i32) -> bool {
        let repr = self.schematic[row_idx as usize][column_idx as usize];
        return repr.is_digit(10);
    }

    fn is_symbol(&self, row_idx: i32, column_idx: i32) -> bool {
        let repr = self.schematic[row_idx as usize][column_idx as usize];
        return !repr.is_digit(10) && repr != '.';
    }

    fn is_gear(&self, row_idx: i32, column_idx: i32) -> bool {
        let repr = self.schematic[row_idx as usize][column_idx as usize];
        return repr == '*';
    }

    fn get_number(&self, location: NumberLocation) -> i32 {
        let (row_idx, (left_idx, right_idx)) = location;
        return self.schematic[row_idx as usize][left_idx as usize..=right_idx as usize]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
    }

    fn get_symbol_locations(&self) -> HashSet<Location> {
        let mut symbol_locations = HashSet::new();
        for row_idx in 0..self.num_rows {
            for column_idx in 0..self.num_columns {
                if self.is_symbol(row_idx, column_idx) {
                    symbol_locations.insert((row_idx, column_idx));
                }
            }
        }

        return symbol_locations;
    }

    fn get_gear_locations(&self) -> HashSet<Location> {
        let mut symbol_locations = HashSet::new();
        for row_idx in 0..self.num_rows {
            for column_idx in 0..self.num_columns {
                if self.is_gear(row_idx, column_idx) {
                    symbol_locations.insert((row_idx, column_idx));
                }
            }
        }

        return symbol_locations;
    }

    fn get_nearby_locations(&self, row_idx: i32, column_idx: i32) -> Vec<Location> {
        let mut nearby_locations = Vec::new();
        for row_offset in -1..2 {
            for column_offset in -1..2 {
                if row_offset == 0 && column_offset == 0 {
                    continue;
                }

                let nearby_row_idx = row_idx + row_offset;
                let nearby_column_idx = column_idx + column_offset;
                if self.in_range(nearby_row_idx, nearby_column_idx) {
                    nearby_locations.push((nearby_row_idx, nearby_column_idx));
                }
            }
        }

        return nearby_locations;
    }

    fn get_number_locations(&self, row_idx: i32, column_idx: i32) -> NumberLocation {
        let mut left_idx = column_idx;
        while 0 < left_idx && self.is_digit(row_idx, left_idx - 1) {
            left_idx -= 1;
        }

        let mut right_idx = column_idx;
        while right_idx < self.num_columns - 1 && self.is_digit(row_idx, right_idx + 1) {
            right_idx += 1;
        }

        return (row_idx, (left_idx, right_idx));
    }

    fn get_part_numbers_around(&self, row_idx: i32, column_idx: i32) -> HashSet<NumberLocation> {
        let mut part_numbers = HashSet::new();
        for (nearby_row_idx, nearby_column_idx) in self.get_nearby_locations(row_idx, column_idx) {
            if self.is_digit(nearby_row_idx, nearby_column_idx) {
                part_numbers.insert(self.get_number_locations(nearby_row_idx, nearby_column_idx));
            }
        }

        return part_numbers;
    }
}

impl EngineSchematic {
    fn solve_part1(&self) -> i32 {
        let mut part_number_locations = HashSet::<NumberLocation>::new();
        let symbol_locations = self.get_symbol_locations();
        for (row_idx, column_idx) in symbol_locations {
            let locations = self.get_part_numbers_around(row_idx, column_idx);
            part_number_locations.extend(locations.iter());
        }

        return part_number_locations
            .iter()
            .map(|location| self.get_number(location.clone()))
            .sum::<i32>();
    }

    fn solve_part2(&self) -> i32 {
        let mut total = 0;
        let gear_locations = self.get_gear_locations();
        for (row_idx, column_idx) in gear_locations {
            let locations = self.get_part_numbers_around(row_idx, column_idx);
            if locations.len() == 2 {
                total += locations
                    .iter()
                    .map(|location| self.get_number(location.clone()))
                    .product::<i32>();
            }
        }

        return total;
    }
}
