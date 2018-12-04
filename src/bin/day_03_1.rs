extern crate regex;
#[macro_use] extern crate lazy_static;

use std::collections::HashSet;
use regex::Regex;

mod utils;

fn main() {
    let contents = utils::load_input("inputs/day3.txt");
    let result = overlaps(&contents);
    println!("{}", result);
}

fn parse(raw_claim: &str) -> Vec<(usize, usize)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let captures = RE.captures_iter(raw_claim).next().unwrap();
    let _id: usize = captures[1].parse().unwrap();
    let start_x: usize = captures[2].parse().unwrap();
    let start_y: usize = captures[3].parse().unwrap();
    let width: usize = captures[4].parse().unwrap();
    let height: usize = captures[5].parse().unwrap();

    let mut positions = Vec::with_capacity(width * height);

    for x in start_x..start_x + width {
        for y in start_y..start_y + height {
            positions.push((x,y));
        }
    }
    positions
}

fn overlaps(raw_claims: &str) -> usize {
    let mut all = HashSet::new();
    let mut overlaps = HashSet::new();
    for raw_claim in raw_claims.lines() {
        for position in parse(raw_claim) {
            if all.contains(&position) && !overlaps.contains(&position) {
                overlaps.insert(position);
            } else {
                all.insert(position);
            }
        }
    }
    overlaps.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_claim() {
        let raw_claim = "#1 @ 1,3: 4x4";
        let claim = parse(&raw_claim);
        let expected = vec!(
            (1,3), (2,3), (3,3), (4,3),
            (1,4), (2,4), (3,4), (4,4),
            (1,5), (2,5), (3,5), (4,5),
            (1,6), (2,6), (3,6), (4,6),
        );
        for pair in expected {
            assert!(claim.contains(&pair));
        }
    }

    #[test]
    fn finds_overlap() {
        let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"].join("\n");
        assert_eq!(4, overlaps(&claims));
    }
}