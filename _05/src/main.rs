use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct EntityRange {
    start: i64,
    length: i64,
}

#[derive(Clone, Copy, Debug)]
struct EntityMap(i64, EntityRange);

impl EntityRange {
    fn end(&self) -> i64 {
        self.start + self.length
    }

    fn overlaps(&self, other: &EntityRange) -> bool {
        if self.start > other.start {
            return other.overlaps(self);
        }

        other.start < self.start + self.length
    }

    fn intersect(&self, other: &EntityRange) -> Option<EntityRange> {
        if !self.overlaps(other) {
            return None;
        }

        let start = self.start.max(other.start);
        let end = self.end().min(other.end());
        Some(EntityRange {
            start,
            length: end - start,
        })
    }
}

impl EntityMap {
    fn shrink(&self, range: EntityRange) -> (EntityRange, Vec<EntityRange>) {
        let EntityMap(destination_start, source_range) = self;
        let mapped_range = EntityRange {
            start: destination_start + (range.start - source_range.start),
            length: range.length,
        };

        let mut origin_ranges = Vec::new();
        if range.start < source_range.start {
            origin_ranges.push(EntityRange {
                start: range.start,
                length: source_range.start - range.start,
            });
        }

        if range.end() > source_range.end() {
            origin_ranges.push(EntityRange {
                start: source_range.end(),
                length: range.end() - source_range.end(),
            });
        }

        (mapped_range, origin_ranges)
    }
}

fn content() -> String {
    std::fs::read_to_string("data/input.txt")
        .unwrap()
        .trim()
        .to_string()
}

fn parse_map(repr: &&str) -> EntityMap {
    let mut values = repr.split(' ').map(|value| value.parse::<i64>().unwrap());
    EntityMap(
        values.next().unwrap(),
        EntityRange {
            start: values.next().unwrap(),
            length: values.next().unwrap(),
        },
    )
}

fn parse_maps(map_reprs: &[&str]) -> Vec<EntityMap> {
    let mut maps = map_reprs.iter().skip(1).map(parse_map).collect::<Vec<_>>();
    maps.sort_by_key(|EntityMap(_, range)| range.start);
    maps
}

fn parse_seed_ranges_p1(repr: &str) -> Vec<EntityRange> {
    repr.split(' ')
        .map(|value| EntityRange {
            start: value.parse::<i64>().unwrap(),
            length: 1,
        })
        .collect::<Vec<_>>()
}

fn parse_seed_ranges_p2(repr: &str) -> Vec<EntityRange> {
    repr.split(' ')
        .map(|value| value.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| EntityRange {
            start: c[0],
            length: c[1],
        })
        .collect::<Vec<_>>()
}

fn entity_map_search(range: &EntityRange, sorted_map: &[EntityMap]) -> EntityMap {
    let mut left = 0;
    let mut right = sorted_map.len();
    while (right - left) > 1 {
        let middle = (left + right) >> 1;
        let EntityMap(_, source_range) = &sorted_map[middle];

        if range.end() <= source_range.start {
            right = middle;
        } else {
            left = middle;
        }
    }

    sorted_map[left]
}

fn entity_ranges_map(entity_ranges: Vec<EntityRange>, maps: &Vec<EntityMap>) -> Vec<EntityRange> {
    let mut mapped_ranges = Vec::new();
    let mut queue = VecDeque::from(entity_ranges);
    while !queue.is_empty() {
        let range = queue.pop_front().unwrap();
        let EntityMap(destination_start, source_range) = entity_map_search(&range, maps);

        match range.intersect(&source_range) {
            Some(intersection) => {
                let (mapped_range, origin_ranges) =
                    EntityMap(destination_start, source_range).shrink(intersection);

                mapped_ranges.push(mapped_range);
                queue.extend(origin_ranges);
            }

            None => {
                mapped_ranges.push(range);
            }
        }
    }

    mapped_ranges
}

fn entity_ranges_merge(mut ranges: Vec<EntityRange>) -> Vec<EntityRange> {
    let mut merged_ranges = Vec::new();
    if ranges.is_empty() {
        return merged_ranges;
    }

    let mut last_range = ranges[0];
    ranges.sort_by_key(|range| range.start);
    for range in ranges.iter().skip(1) {
        if last_range.overlaps(range) {
            last_range.length = range.end() - last_range.start;
            continue;
        }

        merged_ranges.push(last_range);
        last_range = *range;
    }

    merged_ranges.push(last_range);
    merged_ranges
}

fn entity_range_fold(
    entity_ranges: Vec<EntityRange>,
    maps: &Vec<Vec<EntityMap>>,
) -> Vec<EntityRange> {
    maps.iter().fold(entity_ranges, |entity_ranges, maps| {
        entity_ranges_merge(entity_ranges_map(entity_ranges, maps))
    })
}

fn main() {
    let content = content();
    let tokens = content.split('\n').collect::<Vec<_>>();
    let mut iterator = tokens.split(|token| token.is_empty());
    let (_, represent) = iterator.next().unwrap()[0].split_once(": ").unwrap();

    let seed_ranges_p1 = parse_seed_ranges_p1(represent);
    let seed_ranges_p2 = parse_seed_ranges_p2(represent);
    let maps_sequence = iterator
        .map(|map_reprs| parse_maps(map_reprs))
        .collect::<Vec<_>>();

    let entity_ranges_p1 = entity_range_fold(seed_ranges_p1, &maps_sequence);
    let entity_ranges_p2 = entity_range_fold(seed_ranges_p2, &maps_sequence);

    println!("Part 1: {}", entity_ranges_p1[0].start);
    println!("Part 2: {}", entity_ranges_p2[0].start);
}
