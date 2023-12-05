use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Mappings<'a> {
    from: &'a str,
    to: &'a str,
    mappings: Vec<Mapping>
}

#[derive(Debug)]
struct Mapping {
    dst_start: usize,
    src_start: usize,
    range: usize
}

pub fn sum_locations(input: &str) -> usize {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let seeds = input[0]
        .split_once(':')
        .unwrap().1
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mappings = input[1..]
        .iter()
        .map(|block| block.split_once(':').unwrap())
        .map(|(mapping, data)| Mappings::from((mapping, data)))
        .map(|mappings| (mappings.from, mappings))
        .collect::<HashMap<_, _>>();

    traverse_mappings(seeds, mappings)
}

fn traverse_mappings(seeds: Vec<usize>, mappings: HashMap<&str, Mappings>) -> usize {
    let mut locations = Vec::new();
    for seed in seeds {
        let mut current_mapping = mappings.get("seed").unwrap();
        // println!("== seed {seed}");
        let mut mapped_seed = seed;
        while current_mapping.to != "location" {
            mapped_seed = current_mapping.map_seed(mapped_seed);
            // println!("{} {mapped_seed}", current_mapping.to);
            current_mapping = mappings.get(current_mapping.to).unwrap();
        }
        let location = current_mapping.map_seed(mapped_seed);
        // println!("{} {location}", current_mapping.to);
        locations.push(location)
    }

    *locations.iter().min().unwrap()
}

impl<'a> Mappings<'a> {
    fn map_seed(&self, seed: usize) -> usize {
        match self.find_mapping_in_range(seed) {
            Some(mapping) => mapping.map_seed(seed),
            None => seed
        }
    }

    fn find_mapping_in_range(&self, seed: usize) -> Option<&Mapping> {
        self.mappings
            .iter()
            .find(|mapping| mapping.is_in_range(seed))
    }
}

impl Mapping {
    fn map_seed(&self, seed: usize) -> usize {
        self.dst_start + (seed - self.src_start)
    }

    fn is_in_range(&self, seed: usize) -> bool {
        seed >= self.src_start && seed < self.src_start + self.range
    }
}

impl<'a> From<(&'a str, &'a str)> for Mappings<'a> {
    fn from((mapping, data): (&'a str, &'a str)) -> Self {
        let mapping_str = mapping
            .strip_suffix(" map")
            .unwrap()
            .split('-')
            .collect::<Vec<_>>();
        assert_eq!(mapping_str.len(), 3);

        let from = mapping_str[0];
        let to = mapping_str[2];

        let mappings = data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Mapping::from_str(line).unwrap())
            .map(|mapping_range| mapping_range)
            .collect::<Vec<_>>();

        Mappings {
            from,
            to,
            mappings
        }
    }
}

impl FromStr for Mapping {
    type Err = ParseIntError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let range_info = data.split_ascii_whitespace().map(|s| s.parse::<usize>()).collect::<Vec<_>>();
        assert_eq!(range_info.len(), 3);

        Ok(Mapping {
            dst_start: range_info[0].clone()?,
            src_start: range_info[1].clone()?,
            range: range_info[2].clone()?
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::seeds::{Mapping, Mappings, sum_locations};

    #[test]
    fn should_find_closest_location() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        assert_eq!(sum_locations(input), 35);
    }
}